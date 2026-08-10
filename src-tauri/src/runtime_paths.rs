use std::env;
use std::path::PathBuf;

const HARBOR_ROOT_ENV: &str = "HARBOR_ROOT";
const HARBOR_DIRECTORY: &str = "Harbor";
const RUNTIMES_DIRECTORY: &str = "runtimes";
const WORKSPACE_DIRECTORIES: [&str; 7] = [
    "config",
    "runtimes/nodejs",
    "runtimes/php",
    "runtimes/apache",
    "www",
    "logs",
    "cache",
];

pub fn harbor_root() -> PathBuf {
    env::var_os(HARBOR_ROOT_ENV)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(r"C:\").join(HARBOR_DIRECTORY))
}

pub fn runtime_path(runtime_name: &str, version: &str) -> PathBuf {
    harbor_root()
        .join(RUNTIMES_DIRECTORY)
        .join(runtime_name)
        .join(version)
}

pub fn node_path(version: &str) -> PathBuf {
    runtime_path("nodejs", version)
}

pub fn php_path(version: &str) -> PathBuf {
    runtime_path("php", version)
}

pub fn runtime_directory(service: &str) -> Result<PathBuf, String> {
    let runtime_name = match service.as_ref() {
        "Node.js" => "nodejs",
        "PHP" => "php",
        "Apache" => "apache",
        _ => return Err(format!("Unknown runtime service: {service}")),
    };
    Ok(harbor_root().join(RUNTIMES_DIRECTORY).join(runtime_name))
}

pub fn www_directory() -> PathBuf {
    harbor_root().join("www")
}

pub fn bin_directory() -> PathBuf {
    harbor_root().join("bin")
}

pub fn php_alias_path() -> PathBuf {
    bin_directory().join("php.cmd")
}

pub fn initialize_workspace() -> std::io::Result<PathBuf> {
    let root = harbor_root();
    for directory in WORKSPACE_DIRECTORIES {
        std::fs::create_dir_all(root.join(directory))?;
    }
    std::fs::create_dir_all(bin_directory())?;
    Ok(www_directory())
}