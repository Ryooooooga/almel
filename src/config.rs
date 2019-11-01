use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::context::Color;

// OS
#[derive(Debug, Serialize, Deserialize)]
pub struct OsConfig {
    pub linux: OsConfigEntry,
    pub mac: OsConfigEntry,
    pub windows: OsConfigEntry,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsConfigEntry {
    pub background: Color,
    pub foreground: Color,
    pub icon: String,
}

// User
#[derive(Debug, Serialize, Deserialize)]
pub struct UserConfig {
    pub background: Color,
    pub foreground: Color,
    pub display_host: bool,
}

// Status
#[derive(Debug, Serialize, Deserialize)]
pub struct StatusConfig {
    pub icons: StatusConfigIcons,
    pub succeeded: StatusConfigSucceeded,
    pub failed: StatusConfigFailed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusConfigIcons {
    pub succeeded: String,
    pub failed: String,
    pub root: String,
    pub jobs: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusConfigSucceeded {
    pub background: Color,
    pub foreground: Color,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusConfigFailed {
    pub background: Color,
    pub foreground: Color,
    pub display_exit_status: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub os: OsConfig,
    pub user: UserConfig,
    pub status: StatusConfig,
    pub segments: Vec<String>,
}

impl Config {
    pub fn load_from_str(s: &str) -> serde_yaml::Result<Config> {
        serde_yaml::from_str(s)
    }
}

lazy_static! {
    pub static ref DEFAULT_CONFIG_STR: &'static str = include_str!("almel.yaml");
}
