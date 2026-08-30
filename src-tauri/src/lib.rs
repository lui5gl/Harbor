mod runtime_config;
mod runtime_paths;
mod secrets_config;

use std::io::{self, Cursor};
use std::process::{Child, Command};
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use futures_util::StreamExt;

static PHP_PROCESS: OnceLock<Mutex<Option<Child>>> = OnceLock::new();
const PHP_FASTCGI_ADDRESS: &str = "127.0.0.1:9070";

struct CacheEntry<T> {
    data: T,
    timestamp: Instant,
}

static PHP_VERSIONS_CACHE: OnceLock<Mutex<Option<CacheEntry<Vec<String>>>>> = OnceLock::new();
static NODE_VERSIONS_CACHE: OnceLock<Mutex<Option<CacheEntry<Vec<String>>>>> = OnceLock::new();
static APACHE_VERSIONS_CACHE: OnceLock<Mutex<Option<CacheEntry<Vec<String>>>>> = OnceLock::new();
const CACHE_TTL: Duration = Duration::from_secs(30 * 60);

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
    let cache_slot = NODE_VERSIONS_CACHE.get_or_init(|| Mutex::new(None));
    if let Ok(guard) = cache_slot.lock() {
        if let Some(entry) = &*guard {
            if entry.timestamp.elapsed() < CACHE_TTL {
                return Ok(entry.data.clone());
            }
        }
    }

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(7))
        .build()
        .map_err(|e| e.to_string())?;

    let releases_fut = async {
        client
            .get("https://nodejs.org/download/release/index.json")
            .send()
            .await
            .map_err(|error| format!("Unable to fetch Node.js releases: {error}"))?
            .error_for_status()
            .map_err(|error| format!("Node.js release catalog returned an error: {error}"))?
            .json::<Vec<NodeRelease>>()
            .await
            .map_err(|error| format!("Unable to read Node.js release catalog: {error}"))
    };

    let schedule_fut = async {
        client
            .get("https://raw.githubusercontent.com/nodejs/Release/main/schedule.json")
            .send()
            .await
            .map_err(|error| format!("Unable to fetch Node.js release schedule: {error}"))?
            .error_for_status()
            .map_err(|error| format!("Node.js release schedule returned an error: {error}"))?
            .json::<serde_json::Value>()
            .await
            .map_err(|error| format!("Unable to read Node.js release schedule: {error}"))
    };

    let (releases_res, schedule_res) = futures_util::future::join(releases_fut, schedule_fut).await;

    let releases = releases_res?;
    let schedule = schedule_res?;
    let schedule_obj = schedule.as_object();

    let mut versions = releases
        .into_iter()
        .filter(|release| !release.version.is_empty() && !release.date.is_empty())
        .map(|release| {
            let channel = schedule_obj
                .map(|s| node_channel(&release, s))
                .unwrap_or_else(|| "Current".to_string());
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

    if !versions.is_empty() {
        if let Ok(mut guard) = cache_slot.lock() {
            *guard = Some(CacheEntry {
                data: versions.clone(),
                timestamp: Instant::now(),
            });
        }
    }

    Ok(versions)
}

#[tauri::command]
async fn get_php_versions() -> Result<Vec<String>, String> {
    let cache_slot = PHP_VERSIONS_CACHE.get_or_init(|| Mutex::new(None));
    if let Ok(guard) = cache_slot.lock() {
        if let Some(entry) = &*guard {
            if entry.timestamp.elapsed() < CACHE_TTL {
                return Ok(entry.data.clone());
            }
        }
    }

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(7))
        .build()
        .map_err(|e| e.to_string())?;

    let support_cycles_fut = async {
        client
            .get("https://endoflife.date/api/php.json")
            .send()
            .await
            .map_err(|error| format!("Unable to fetch PHP support schedule: {error}"))?
            .error_for_status()
            .map_err(|error| format!("PHP support schedule returned an error: {error}"))?
            .json::<Vec<PhpSupportCycle>>()
            .await
            .map_err(|error| format!("Unable to read PHP support schedule: {error}"))
    };

    let branches = ["8", "7", "5"];
    let branch_futs = branches.iter().map(|branch| {
        let client = client.clone();
        async move {
            let url = format!(
                "https://www.php.net/releases/index.php?json=1&version={branch}&max=1000"
            );
            client
                .get(url)
                .send()
                .await
                .map_err(|error| format!("Unable to fetch PHP releases: {error}"))?
                .error_for_status()
                .map_err(|error| format!("PHP release catalog returned an error: {error}"))?
                .json::<std::collections::HashMap<String, serde_json::Value>>()
                .await
                .map_err(|error| format!("Unable to read PHP release catalog: {error}"))
        }
    });

    let (support_cycles_res, branch_results) = futures_util::future::join(
        support_cycles_fut,
        futures_util::future::join_all(branch_futs),
    )
    .await;

    let support_cycles = support_cycles_res.unwrap_or_default();
    let mut versions = std::collections::HashSet::new();

    for branch_res in branch_results {
        if let Ok(releases) = branch_res {
            for (version, _) in releases {
                if semver::Version::parse(&version).is_ok() {
                    let channel = php_channel(&version, &support_cycles);
                    versions.insert(format!("{version} ({channel})"));
                }
            }
        }
    }

    let mut versions = versions.into_iter().collect::<Vec<_>>();
    versions.sort_by(|left, right| compare_versions(left, right));

    if !versions.is_empty() {
        if let Ok(mut guard) = cache_slot.lock() {
            *guard = Some(CacheEntry {
                data: versions.clone(),
                timestamp: Instant::now(),
            });
        }
    }

    Ok(versions)
}

#[tauri::command]
async fn get_apache_versions() -> Result<Vec<String>, String> {
    let cache_slot = APACHE_VERSIONS_CACHE.get_or_init(|| Mutex::new(None));
    if let Ok(guard) = cache_slot.lock() {
        if let Some(entry) = &*guard {
            if entry.timestamp.elapsed() < CACHE_TTL {
                return Ok(entry.data.clone());
            }
        }
    }

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(7))
        .build()
        .map_err(|e| e.to_string())?;

    let index = client
        .get("https://downloads.apache.org/httpd/")
        .send()
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

    if !versions.is_empty() {
        if let Ok(mut guard) = cache_slot.lock() {
            *guard = Some(CacheEntry {
                data: versions.clone(),
                timestamp: Instant::now(),
            });
        }
    }

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

fn generate_php_archive_urls(version: &str) -> Vec<String> {
    let base_dirs = [
        "https://windows.php.net/downloads/releases",
        "https://windows.php.net/downloads/releases/archives",
    ];

    let major = version.split('.').next().unwrap_or("8");
    let toolchains: &[&str] = match major {
        "8" => &["vs17", "vs16", "VC15", "vc15", "VC14"],
        "7" => &["vc15", "VC15", "vc14", "VC14", "vs16"],
        "5" => &["VC11", "vc11", "VC9", "vc9"],
        _ => &["vs17", "vs16", "vc15", "vc14", "vc11", "vc9"],
    };

    let archs = ["x64", "x86"];

    let mut urls = Vec::new();
    for base in base_dirs {
        for toolchain in toolchains {
            for arch in archs {
                urls.push(format!("{base}/php-{version}-Win32-{toolchain}-{arch}.zip"));
                urls.push(format!("{base}/php-{version}-nts-Win32-{toolchain}-{arch}.zip"));
            }
        }
    }
    urls
}

#[tauri::command]
async fn install_php(app: tauri::AppHandle, version: String) -> Result<String, String> {
    let parsed_version = semver::Version::parse(version.trim_start_matches('v'))
        .map_err(|_| format!("Invalid PHP version: {version}"))?;
    let version = parsed_version.to_string();

    let target_directory = runtime_paths::php_path(&version);
    if target_directory.join("php.exe").is_file() {
        return Ok(target_directory.to_string_lossy().into_owned());
    }

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(15))
        .build()
        .map_err(|e| e.to_string())?;

    let candidate_urls = generate_php_archive_urls(&version);
    let mut verified_url: Option<String> = None;

    for chunk in candidate_urls.chunks(8) {
        let requests = chunk.iter().map(|url| {
            let client = client.clone();
            let url_str = url.clone();
            async move {
                if let Ok(res) = client.head(&url_str).send().await {
                    if res.status().is_success() {
                        return Some(url_str);
                    }
                }
                None
            }
        });
        let results = futures_util::future::join_all(requests).await;
        for res in results {
            if let Some(valid_url) = res {
                verified_url = Some(valid_url);
                break;
            }
        }
        if verified_url.is_some() {
            break;
        }
    }

    let target_url = verified_url.ok_or_else(|| {
        format!("No official Windows PHP archive was found on windows.php.net for {version}")
    })?;

    let response = client
        .get(&target_url)
        .send()
        .await
        .map_err(|error| format!("Unable to download PHP {version}: {error}"))?
        .error_for_status()
        .map_err(|error| format!("Download server returned an error: {error}"))?;

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
        let _ = tauri::Emitter::emit(
            &app,
            "runtime-download-progress",
            DownloadProgress {
                service: "PHP",
                version: &version,
                progress,
            },
        );
    }

    std::fs::create_dir_all(&target_directory).map_err(format_io_error)?;
    if let Err(error) = extract_zip(&archive, &target_directory) {
        let _ = std::fs::remove_dir_all(&target_directory);
        return Err(error);
    }
    let _ = tauri::Emitter::emit(
        &app,
        "runtime-download-progress",
        DownloadProgress {
            service: "PHP",
            version: &version,
            progress: 100,
        },
    );
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
fn get_active_runtimes() -> Result<runtime_config::ActiveRuntimes, String> {
    runtime_config::read_active_runtimes().map_err(format_io_error)
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
fn set_active_php_version(version: String) -> Result<String, String> {
    let php_path = runtime_paths::php_path(&version);
    if !php_path.is_dir() {
        return Err(format!("PHP version is not installed: {version}"));
    }

    configure_php_cli_alias(version.clone())?;
    runtime_config::write_active_php_version(&version).map_err(format_io_error)?;
    Ok(php_path.to_string_lossy().into_owned())
}

#[tauri::command]
fn set_active_apache_version(version: String) -> Result<String, String> {
    let apache_path = runtime_paths::runtime_path("apache", &version);
    if !apache_path.is_dir() {
        return Err(format!("Apache version is not installed: {version}"));
    }

    runtime_config::write_active_apache_version(&version).map_err(format_io_error)?;
    Ok(apache_path.to_string_lossy().into_owned())
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

    // Terminate existing process if already running so we can switch version cleanly
    if let Some(mut existing_child) = process.take() {
        let _ = existing_child.kill();
        let _ = existing_child.wait();
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
    if let Ok(env_key) = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER).open_subkey_with_flags("Environment", winreg::enums::KEY_READ | winreg::enums::KEY_WRITE) {
        let current_path: String = env_key.get_value("Path").unwrap_or_default();
        let parts: Vec<&str> = current_path.split(';').filter(|p| !p.trim().is_empty() && *p != bin_path).collect();
        let mut new_parts = parts;
        new_parts.push(&bin_path);
        let new_path = new_parts.join(";");
        let _ = env_key.set_value("Path", &new_path);
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

static IS_EXITING: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn perform_app_exit(app: &tauri::AppHandle) {
    IS_EXITING.store(true, std::sync::atomic::Ordering::SeqCst);
    let _ = stop_php();
    app.exit(0);
}

#[tauri::command]
fn show_main_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(quick_window) = app.get_webview_window("quick-tray") {
        let _ = quick_window.hide();
    }
    if let Some(main_window) = app.get_webview_window("main") {
        let _ = main_window.show();
        let _ = main_window.unminimize();
        let _ = main_window.set_focus();
    }
    Ok(())
}

#[tauri::command]
fn hide_quick_tray(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(quick_window) = app.get_webview_window("quick-tray") {
        let _ = quick_window.hide();
    }
    Ok(())
}

#[tauri::command]
fn toggle_quick_tray(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(quick_window) = app.get_webview_window("quick-tray") {
        if quick_window.is_visible().unwrap_or(false) {
            let _ = quick_window.hide();
        } else {
            show_or_toggle_quick_window(&app);
        }
    }
    Ok(())
}

#[tauri::command]
fn exit_app(app: tauri::AppHandle) {
    perform_app_exit(&app);
}

use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, WindowEvent,
};

fn show_or_toggle_quick_window(app: &tauri::AppHandle) {
    if let Some(quick_window) = app.get_webview_window("quick-tray") {
        if quick_window.is_visible().unwrap_or(false) {
            let _ = quick_window.hide();
        } else {
            if let Ok(Some(monitor)) = quick_window.primary_monitor() {
                let size = monitor.size();
                let scale_factor = monitor.scale_factor();
                let win_width = (400.0 * scale_factor) as i32;
                let win_height = (540.0 * scale_factor) as i32;
                let x = (size.width as i32) - win_width - (16.0 * scale_factor) as i32;
                let y = (size.height as i32) - win_height - (56.0 * scale_factor) as i32;
                let _ = quick_window.set_position(tauri::Position::Physical(tauri::PhysicalPosition { x, y }));
            }
            let _ = quick_window.show();
            let _ = quick_window.unminimize();
            let _ = quick_window.set_focus();
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let show_main_item = MenuItem::with_id(app, "open_main", "Abrir Harbor", true, None::<&str>)?;
            let quick_env_item = MenuItem::with_id(app, "open_quick_env", "Acceso Rápido (Bandeja)...", true, None::<&str>)?;
            let sep = PredefinedMenuItem::separator(app)?;
            let quit_item = MenuItem::with_id(app, "quit", "Salir de Harbor", true, None::<&str>)?;

            let tray_menu = Menu::with_items(
                app,
                &[
                    &show_main_item,
                    &quick_env_item,
                    &sep,
                    &quit_item,
                ],
            )?;

            let mut tray_builder = TrayIconBuilder::with_id("harbor-tray")
                .tooltip("Harbor - Desktop & Environment Manager")
                .menu(&tray_menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "open_main" => {
                        let _ = show_main_window(app.clone());
                    }
                    "open_quick_env" => {
                        show_or_toggle_quick_window(app);
                    }
                    "quit" => {
                        perform_app_exit(app);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        show_or_toggle_quick_window(app);
                    }
                });

            if let Some(icon) = app.default_window_icon() {
                tray_builder = tray_builder.icon(icon.clone());
            }

            tray_builder.build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            match event {
                WindowEvent::CloseRequested { api, .. } => {
                    if !IS_EXITING.load(std::sync::atomic::Ordering::SeqCst) {
                        api.prevent_close();
                        let _ = window.hide();
                    }
                }
                WindowEvent::Focused(false) => {
                    if window.label() == "quick-tray" {
                        let _ = window.hide();
                    }
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            get_node_versions,
            get_php_versions,
            get_apache_versions,
            get_installed_versions,
            get_active_runtimes,
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
            set_active_php_version,
            set_active_apache_version,
            load_secret_profiles,
            save_secret_profiles,
            activate_secret_profile_for_powershell,
            show_main_window,
            hide_quick_tray,
            toggle_quick_tray,
            exit_app
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
