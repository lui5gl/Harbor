mod runtime_config;
mod runtime_paths;

use std::io;

#[derive(serde::Deserialize, serde::Serialize)]
struct NodeRelease {
    version: String,
    date: String,
    lts: serde_json::Value,
}

fn node_channel(lts: &serde_json::Value) -> String {
    match lts.as_str().filter(|name| !name.is_empty() && *name != "false") {
        Some(name) => format!("LTS - {name}"),
        None => "Current".to_string(),
    }
}

#[tauri::command]
async fn get_node_versions() -> Result<Vec<String>, String> {
    let releases = reqwest::get("https://nodejs.org/download/release/index.json")
        .await
        .map_err(|error| format!("Unable to fetch Node.js releases: {error}"))?
        .error_for_status()
        .map_err(|error| format!("Node.js release catalog returned an error: {error}"))?
        .json::<Vec<NodeRelease>>()
        .await
        .map_err(|error| format!("Unable to read Node.js release catalog: {error}"))?;

    let mut versions = releases
        .into_iter()
        .filter(|release| !release.version.is_empty() && !release.date.is_empty())
        .map(|release| {
            format!("{} ({})", release.version, node_channel(&release.lts))
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
                .filter(|version| semver::Version::parse(version).is_ok()),
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

fn format_io_error(error: io::Error) -> String {
    format!("Unable to update Harbor runtime configuration: {error}")
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
            set_active_node_version
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
