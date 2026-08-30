mod runtime_config;
mod runtime_paths;
mod secrets_config;

use std::io::{self, Cursor};
use std::process::{Child, Command};
use std::sync::{Mutex, OnceLock};

use futures_util::StreamExt;

static PHP_PROCESS: OnceLock<Mutex<Option<Child>>> = OnceLock::new();
const PHP_FASTCGI_ADDRESS: &str = "127.0.0.1:9070";

#[derive(serde::Deserialize, serde::Serialize)]
struct NodeRelease {
    version: String,
    date: String,
    lts: serde_json::Value,
}

#[derive(serde::Deserialize)]
struct PhpSupportCycle {
    cycle: String,
    support: String,
    eol: String,
}

fn php_channel(version: &str, support_cycles: &[PhpSupportCycle]) -> String {
    let cycle = version.split('.').take(2).collect::<Vec<_>>().join(".");
    let today = chrono::Utc::now().date_naive().to_string();
    let support_cycle = support_cycles.iter().find(|entry| entry.cycle == cycle);

    let Some(support_cycle) = support_cycle else {
        return "Unknown".to_string();
    };
    if today.as_str() <= support_cycle.support.as_str() {
        return "Active".to_string();
    }
    if today.as_str() <= support_cycle.eol.as_str() {
        return "Security".to_string();
    }
    "EOL".to_string()
}

fn node_channel(release: &NodeRelease, schedule: &serde_json::Map<String, serde_json::Value>) -> String {
    let major_version = release
        .version
        .split('.')
        .next()
        .unwrap_or_default()
        .trim_start_matches('v');
    let schedule_key = format!("v{major_version}");
    let schedule_entry = schedule.get(&schedule_key).and_then(serde_json::Value::as_object);
    let today = chrono::Utc::now().date_naive().to_string();
    let is_eol = schedule_entry
        .and_then(|entry| entry.get("eol"))
        .and_then(serde_json::Value::as_str)
        .is_some_and(|eol_date| eol_date <= today.as_str());

    if is_eol {
        return "EOL".to_string();
    }

    match release.lts.as_str().filter(|name| !name.is_empty() && *name != "false") {
        Some(name) => format!("LTS - {name}"),
        None => "Current".to_string(),
    }
}

#[tauri::command]
async fn get_node_versions() -> Result<Vec<String>, String> {
    let client = reqwest::Client::new();
    let releases = client
        .get("https://nodejs.org/download/release/index.json")
        .send()
        .await
        .map_err(|error| format!("Unable to fetch Node.js releases: {error}"))?
        .error_for_status()
        .map_err(|error| format!("Node.js release catalog returned an error: {error}"))?
        .json::<Vec<NodeRelease>>()
        .await
        .map_err(|error| format!("Unable to read Node.js release catalog: {error}"))?;
    let schedule = client
        .get("https://raw.githubusercontent.com/nodejs/Release/main/schedule.json")
        .send()
        .await
        .map_err(|error| format!("Unable to fetch Node.js release schedule: {error}"))?
        .error_for_status()
        .map_err(|error| format!("Node.js release schedule returned an error: {error}"))?
        .json::<serde_json::Value>()
        .await
        .map_err(|error| format!("Unable to read Node.js release schedule: {error}"))?;
    let schedule = schedule
        .as_object()
        .ok_or_else(|| "Node.js release schedule has an invalid format".to_string())?;

    let mut versions = releases
        .into_iter()
        .filter(|release| !release.version.is_empty() && !release.date.is_empty())
        .map(|release| {
            let channel = node_channel(&release, schedule);
            format!("{} ({channel})", release.version)
        })
        .collect::<Vec<_>>();
    versions.sort_by(|left, right| {
        let left_version = left.split(' ').next().unwrap_or_default();
        let right_version = right.split(' ').next().unwrap_or_default();
        let left_version = semver::Version::parse(left_version.trim_start_matches('v'));
        let right_version = semver::Version::parse(right_version.trim_start_matches('v'));
        match (left_version, right_version) {
            (Ok(left_version), Ok(right_version)) => right_version.cmp(&left_version),
            (Err(_), Err(_)) => std::cmp::Ordering::Equal,
            (Err(_), Ok(_)) => std::cmp::Ordering::Greater,
            (Ok(_), Err(_)) => std::cmp::Ordering::Less,
        }
    });
    Ok(versions)
}

#[tauri::command]
async fn get_php_versions() -> Result<Vec<String>, String> {
    let support_cycles = reqwest::get("https://endoflife.date/api/php.json")
        .await
        .map_err(|error| format!("Unable to fetch PHP support schedule: {error}"))?
        .error_for_status()
        .map_err(|error| format!("PHP support schedule returned an error: {error}"))?
        .json::<Vec<PhpSupportCycle>>()
        .await
        .map_err(|error| format!("Unable to read PHP support schedule: {error}"))?;
    let mut versions = std::collections::HashSet::new();
    for branch in ["8", "7", "5", "4", "3"] {
        let url = format!(
            "https://www.php.net/releases/index.php?json=1&version={branch}&max=1000"
        );
        let releases = reqwest::get(url)
            .await
            .map_err(|error| format!("Unable to fetch PHP releases: {error}"))?
            .error_for_status()
            .map_err(|error| format!("PHP release catalog returned an error: {error}"))?
            .json::<std::collections::HashMap<String, serde_json::Value>>()
            .await
            .map_err(|error| format!("Unable to read PHP release catalog: {error}"))?;

        versions.extend(
            releases
                .into_keys()
                .filter(|version| semver::Version::parse(version).is_ok())
                .map(|version| format!("{version} ({})", php_channel(&version, &support_cycles))),
        );
    }

    let mut versions = versions.into_iter().collect::<Vec<_>>();
    versions.sort_by(|left, right| compare_versions(left, right));
    Ok(versions)
}

#[tauri::command]
async fn get_apache_versions() -> Result<Vec<String>, String> {
    let index = reqwest::get("https://downloads.apache.org/httpd/")
        .await
        .map_err(|error| format!("Unable to fetch Apache releases: {error}"))?
        .error_for_status()
        .map_err(|error| format!("Apache release catalog returned an error: {error}"))?
        .text()
        .await
        .map_err(|error| format!("Unable to read Apache release catalog: {error}"))?;

    let mut versions = index
        .split("httpd-")
        .skip(1)
        .filter_map(|entry| entry.split(['\"', '\'', '<', '>']).next())
        .filter(|version| version.starts_with("2.") && (version.ends_with(".tar.gz") || version.ends_with(".tar.bz2")))
        .map(|version| {
            version
                .trim_end_matches(".tar.gz")
                .trim_end_matches(".tar.bz2")
                .to_owned()
        })
        .collect::<Vec<_>>();
    versions.sort_by(|left, right| compare_versions(left, right));
    versions.dedup();
    Ok(versions)
}

#[tauri::command]
fn get_installed_versions(service: String) -> Result<Vec<String>, String> {
    let runtime_directory = runtime_paths::runtime_directory(&service)?;
    if !runtime_directory.is_dir() {
        return Ok(Vec::new());
    }

    let mut versions = std::fs::read_dir(runtime_directory)
        .map_err(format_io_error)?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_dir())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .collect::<Vec<_>>();
    versions.sort_by(|left, right| compare_versions(left, right));
    Ok(versions)
}

#[tauri::command]
fn remove_runtime(service: String, version: String) -> Result<(), String> {
    let parsed_version = semver::Version::parse(version.trim_start_matches('v'))
        .map_err(|_| format!("Invalid {service} version: {version}"))?;
    let normalized_version = parsed_version.to_string();
    let runtime_directory = runtime_paths::runtime_directory(&service)?;
    let target_directory = runtime_directory.join(&normalized_version);
    if !target_directory.is_dir() {
        return Err(format!("{service} {normalized_version} is not installed"));
    }
    std::fs::remove_dir_all(&target_directory).map_err(format_io_error)
}

#[tauri::command]
fn initialize_harbor_workspace() -> Result<String, String> {
    runtime_paths::initialize_workspace()
        .map(|path| path.to_string_lossy().into_owned())
        .map_err(format_io_error)
}

#[tauri::command]
async fn install_php(app: tauri::AppHandle, version: String) -> Result<String, String> {
    let parsed_version = semver::Version::parse(&version)
        .map_err(|_| format!("Invalid PHP version: {version}"))?;
    if parsed_version.major < 8 {
        return Err("Automatic Windows installation currently supports PHP 8.x binaries only".to_string());
    }

    let target_directory = runtime_paths::php_path(&version);
    if target_directory.is_dir() {
        return Ok(target_directory.to_string_lossy().into_owned());
    }

    let archive_urls = [
        format!("https://windows.php.net/downloads/releases/archives/php-{version}-Win32-vs17-x64.zip"),
        format!("https://windows.php.net/downloads/releases/archives/php-{version}-Win32-vs16-x64.zip"),
    ];
    let mut response = None;
    for archive_url in archive_urls {
        let candidate = reqwest::get(archive_url).await;
        if let Ok(candidate) = candidate {
            if candidate.status().is_success() {
                response = Some(candidate);
                break;
            }
        }
    }
    let response = response.ok_or_else(|| format!("No official Windows PHP archive was found for {version}"))?;
    let total_bytes = response.content_length().unwrap_or_default();
    let mut downloaded_bytes = 0_u64;
    let mut archive = Vec::new();
    let mut stream = response.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|error| format!("Unable to read PHP {version} download: {error}"))?;
        downloaded_bytes += chunk.len() as u64;
        archive.extend_from_slice(&chunk);
        let progress = if total_bytes == 0 {
            1
        } else {
            ((downloaded_bytes * 100) / total_bytes).min(100) as u8
        };
        let _ = tauri::Emitter::emit(&app, "runtime-download-progress", DownloadProgress { service: "PHP", version: &version, progress });
    }

    std::fs::create_dir_all(&target_directory).map_err(format_io_error)?;
    if let Err(error) = extract_zip(&archive, &target_directory) {
        let _ = std::fs::remove_dir_all(&target_directory);
        return Err(error);
    }
    let _ = tauri::Emitter::emit(&app, "runtime-download-progress", DownloadProgress { service: "PHP", version: &version, progress: 100 });
    Ok(target_directory.to_string_lossy().into_owned())
}

async fn download_archive(
    app: &tauri::AppHandle,
    service: &'static str,
    version: &str,
    url: &str,
) -> Result<Vec<u8>, String> {
    let response = reqwest::get(url)
        .await
        .map_err(|error| format!("Unable to download {service} {version}: {error}"))?
        .error_for_status()
        .map_err(|error| format!("No Windows archive was found for {service} {version}: {error}"))?;
    let total_bytes = response.content_length().unwrap_or_default();
    let mut downloaded_bytes = 0_u64;
    let mut archive = Vec::new();
    let mut stream = response.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|error| format!("Unable to read {service} {version} download: {error}"))?;
        downloaded_bytes += chunk.len() as u64;
        archive.extend_from_slice(&chunk);
        let progress = if total_bytes == 0 { 0 } else { ((downloaded_bytes * 100) / total_bytes).min(100) as u8 };
        let _ = tauri::Emitter::emit(app, "runtime-download-progress", DownloadProgress { service, version, progress });
    }
    Ok(archive)
}

#[tauri::command]
async fn install_node(app: tauri::AppHandle, version: String) -> Result<String, String> {
    let parsed_version = semver::Version::parse(version.trim_start_matches('v'))
        .map_err(|_| format!("Invalid Node.js version: {version}"))?;
    let version = parsed_version.to_string();
    let target_directory = runtime_paths::node_path(&version);
    if target_directory.join("node.exe").is_file() {
        return Ok(target_directory.to_string_lossy().into_owned());
    }
    if target_directory.exists() {
        std::fs::remove_dir_all(&target_directory).map_err(format_io_error)?;
    }
    let archive_url = format!("https://nodejs.org/dist/v{version}/node-v{version}-win-x64.zip");
    let archive = download_archive(&app, "Node.js", &version, &archive_url).await?;
    std::fs::create_dir_all(&target_directory).map_err(format_io_error)?;
    if let Err(error) = extract_zip(&archive, &target_directory) {
        let _ = std::fs::remove_dir_all(&target_directory);
        return Err(format!("Unable to extract Node.js {version}: {error}"));
    }
    let extracted_directory = target_directory.join(format!("node-v{version}-win-x64"));
    if let Err(error) = flatten_directory(&extracted_directory, &target_directory) {
        let _ = std::fs::remove_dir_all(&target_directory);
        return Err(format!("Unable to prepare Node.js {version}: {error}"));
    }
    let _ = std::fs::remove_dir_all(extracted_directory);
    let _ = tauri::Emitter::emit(&app, "runtime-download-progress", DownloadProgress { service: "Node.js", version: &version, progress: 100 });
    Ok(target_directory.to_string_lossy().into_owned())
}

#[tauri::command]
async fn install_apache(app: tauri::AppHandle, version: String) -> Result<String, String> {
    let parsed_version = semver::Version::parse(version.trim_start_matches('v'))
        .map_err(|_| format!("Invalid Apache version: {version}"))?;
    let version = parsed_version.to_string();
    let target_directory = runtime_paths::runtime_path("apache", &version);
    if target_directory.is_dir() { return Ok(target_directory.to_string_lossy().into_owned()); }
    let archive_url = format!("https://www.apachelounge.com/download/VS17/binaries/httpd-{version}-win64-VS17.zip");
    let archive = download_archive(&app, "Apache", &version, &archive_url).await?;
    std::fs::create_dir_all(&target_directory).map_err(format_io_error)?;
    extract_zip(&archive, &target_directory)?;
    let extracted_directory = target_directory.join("Apache24");
    flatten_directory(&extracted_directory, &target_directory)?;
    let _ = std::fs::remove_dir_all(extracted_directory);
    Ok(target_directory.to_string_lossy().into_owned())
}

#[derive(serde::Serialize, Clone)]
struct DownloadProgress<'a> {
    service: &'static str,
    version: &'a str,
    progress: u8,
}

fn extract_zip(archive: &[u8], target_directory: &std::path::Path) -> Result<(), String> {
    let mut zip = zip::ZipArchive::new(Cursor::new(archive))
        .map_err(|error| format!("Unable to open PHP archive: {error}"))?;
    for index in 0..zip.len() {
        let mut entry = zip
            .by_index(index)
            .map_err(|error| format!("Unable to read PHP archive: {error}"))?;
        let entry_path = std::path::Path::new(entry.name());
        if entry_path
            .components()
            .any(|component| matches!(component, std::path::Component::ParentDir))
        {
            return Err("PHP archive contains an unsafe path".to_string());
        }
        let output_path = target_directory.join(entry_path);
        if entry.is_dir() {
            std::fs::create_dir_all(output_path).map_err(format_io_error)?;
            continue;
        }
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent).map_err(format_io_error)?;
        }
        let mut output = std::fs::File::create(output_path).map_err(format_io_error)?;
        io::copy(&mut entry, &mut output).map_err(format_io_error)?;
    }
    Ok(())
}

fn flatten_directory(source: &std::path::Path, target: &std::path::Path) -> Result<(), String> {
    if !source.is_dir() { return Err(format!("Archive has an unexpected directory layout: {}", source.display())); }
    for entry in std::fs::read_dir(source).map_err(format_io_error)? {
        let entry = entry.map_err(format_io_error)?;
        let destination = target.join(entry.file_name());
        std::fs::rename(entry.path(), destination).map_err(format_io_error)?;
    }
    Ok(())
}

fn compare_versions(left: &str, right: &str) -> std::cmp::Ordering {
    let left_version = semver::Version::parse(left.trim_start_matches('v'));
    let right_version = semver::Version::parse(right.trim_start_matches('v'));
    match (left_version, right_version) {
        (Ok(left_version), Ok(right_version)) => right_version.cmp(&left_version),
        (Err(_), Err(_)) => std::cmp::Ordering::Equal,
        (Err(_), Ok(_)) => std::cmp::Ordering::Greater,
        (Ok(_), Err(_)) => std::cmp::Ordering::Less,
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn set_active_node_version(version: String) -> Result<String, String> {
    let node_path = runtime_paths::node_path(&version);
    if !node_path.is_dir() {
        return Err(format!("Node.js version is not installed: {version}"));
    }

    runtime_config::write_active_node_version(&version).map_err(format_io_error)?;
    Ok(node_path.to_string_lossy().into_owned())
}

#[tauri::command]
fn start_php(version: String) -> Result<String, String> {
    let php_binary = runtime_paths::php_path(&version).join("php-cgi.exe");
    if !php_binary.is_file() {
        return Err(format!("PHP {version} is not installed correctly: php-cgi.exe was not found"));
    }

    let process_slot = PHP_PROCESS.get_or_init(|| Mutex::new(None));
    let mut process = process_slot
        .lock()
        .map_err(|_| "Unable to access PHP process state".to_string())?;
    if process.as_mut().is_some_and(|child| child.try_wait().ok().flatten().is_none()) {
        return Ok(PHP_FASTCGI_ADDRESS.to_string());
    }

    let child = Command::new(php_binary)
        .args(["-b", PHP_FASTCGI_ADDRESS])
        .current_dir(runtime_paths::php_path(&version))
        .spawn()
        .map_err(|error| format!("Unable to start PHP {version}: {error}"))?;
    *process = Some(child);
    Ok(PHP_FASTCGI_ADDRESS.to_string())
}

#[tauri::command]
fn stop_php() -> Result<(), String> {
    let process_slot = PHP_PROCESS.get_or_init(|| Mutex::new(None));
    let mut process = process_slot
        .lock()
        .map_err(|_| "Unable to access PHP process state".to_string())?;
    if let Some(mut child) = process.take() {
        child
            .kill()
            .map_err(|error| format!("Unable to stop PHP: {error}"))?;
    }
    Ok(())
}

#[tauri::command]
fn get_php_status() -> Result<bool, String> {
    let process_slot = PHP_PROCESS.get_or_init(|| Mutex::new(None));
    let mut process = process_slot
        .lock()
        .map_err(|_| "Unable to access PHP process state".to_string())?;
    let is_running = process
        .as_mut()
        .is_some_and(|child| child.try_wait().ok().flatten().is_none());
    if !is_running {
        *process = None;
    }
    Ok(is_running)
}

#[tauri::command]
fn get_php_cli_path(version: String) -> Result<String, String> {
    let php_binary = runtime_paths::php_path(&version).join("php.exe");
    if !php_binary.is_file() {
        return Err(format!("PHP {version} CLI executable was not found"));
    }
    Ok(php_binary.to_string_lossy().into_owned())
}

#[tauri::command]
fn configure_php_cli_alias(version: String) -> Result<String, String> {
    let php_binary = runtime_paths::php_path(&version).join("php.exe");
    if !php_binary.is_file() {
        return Err(format!("PHP {version} CLI executable was not found"));
    }

    runtime_paths::initialize_workspace().map_err(format_io_error)?;
    let alias_contents = format!("@echo off\r\n\"{}\" %*\r\n", php_binary.display());
    std::fs::write(runtime_paths::php_alias_path(), alias_contents).map_err(format_io_error)?;

    let bin_path = runtime_paths::bin_directory().to_string_lossy().into_owned();
    let script = format!(
        "$current = [Environment]::GetEnvironmentVariable('Path', 'User'); $parts = @($current -split ';' | Where-Object {{ $_ -and ($_ -ne '{}') }}); [Environment]::SetEnvironmentVariable('Path', (($parts + '{}') -join ';'), 'User')",
        bin_path.replace('\'', "''"),
        bin_path.replace('\'', "''")
    );
    let status = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-ExecutionPolicy", "Bypass", "-Command", &script])
        .status()
        .map_err(|error| format!("Unable to update the user PATH: {error}"))?;
    if !status.success() {
        return Err("Unable to update the user PATH".to_string());
    }
    Ok(bin_path)
}

fn format_io_error(error: io::Error) -> String {
    format!("Unable to update Harbor runtime configuration: {error}")
}

#[tauri::command]
async fn load_secret_profiles() -> Result<secrets_config::SecretsConfiguration, String> {
    tauri::async_runtime::spawn_blocking(secrets_config::load)
        .await
        .map_err(|error| format!("Unable to load secret profiles: {error}"))?
}

#[tauri::command]
async fn save_secret_profiles(configuration: secrets_config::SecretsConfiguration) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || secrets_config::save(configuration))
        .await
        .map_err(|error| format!("Unable to save secret profiles: {error}"))?
}

#[tauri::command]
async fn activate_secret_profile_for_powershell(profile_id: u64) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || secrets_config::activate_powershell_profile(profile_id))
        .await
        .map_err(|error| format!("Unable to activate secret profile: {error}"))?
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_node_versions,
            get_php_versions,
            get_apache_versions,
            get_installed_versions,
            remove_runtime,
            initialize_harbor_workspace,
            install_php,
            install_node,
            install_apache,
            start_php,
            stop_php,
            get_php_status,
            get_php_cli_path,
            configure_php_cli_alias,
            set_active_node_version,
            load_secret_profiles,
            save_secret_profiles,
            activate_secret_profile_for_powershell
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
