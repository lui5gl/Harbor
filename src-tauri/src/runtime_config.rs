use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;

use crate::runtime_paths::harbor_root;

const CONFIG_DIRECTORY: &str = "config";
const CONFIG_FILE: &str = "active-runtimes.json";

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ActiveRuntimes {
    pub nodejs: Option<String>,
}

pub fn read_active_runtimes() -> io::Result<ActiveRuntimes> {
    let config_path = config_path();
    if !config_path.exists() {
        return Ok(ActiveRuntimes::default());
    }

    let contents = fs::read_to_string(config_path)?;
    serde_json::from_str(&contents).map_err(io::Error::other)
}

pub fn write_active_node_version(version: &str) -> io::Result<()> {
    let config_directory = harbor_root().join(CONFIG_DIRECTORY);
    fs::create_dir_all(&config_directory)?;

    let mut active_runtimes = read_active_runtimes()?;
    active_runtimes.nodejs = Some(version.to_owned());
    let contents = serde_json::to_string_pretty(&active_runtimes).map_err(io::Error::other)?;
    fs::write(config_directory.join(CONFIG_FILE), contents)
}

fn config_path() -> PathBuf {
    harbor_root().join(CONFIG_DIRECTORY).join(CONFIG_FILE)
}