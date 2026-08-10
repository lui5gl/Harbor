use std::env;
use std::path::PathBuf;

const HARBOR_ROOT_ENV: &str = "HARBOR_ROOT";
const HARBOR_DIRECTORY: &str = "Harbor";
const RUNTIMES_DIRECTORY: &str = "runtimes";

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