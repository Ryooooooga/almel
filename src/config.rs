use failure::Fail;
use serde::{Deserialize, Serialize};

use crate::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct OsEntry {
    pub background: String,
    pub foreground: String,
    pub icon: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Os {
    pub linux: OsEntry,
    pub mac: OsEntry,
    pub windows: OsEntry,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub background: String,
    pub foreground: String,
    pub display_host: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Git {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Newline {}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusSucceeded {
    pub background: String,
    pub foreground: String,
    pub icon: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusFailed {
    pub background: String,
    pub foreground: String,
    pub icon: String,
    pub display_exit_code: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub root_icon: String,
    pub job_icon: String,
    pub succeeded: StatusSucceeded,
    pub failed: StatusFailed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SegmentSeparators {
    pub left_solid: String,
}

type Segments = Vec<String>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub os: Os,
    pub user: User,
    pub git: Git,
    pub newline: Newline,
    pub status: Status,
    pub segment_separators: SegmentSeparators,
    pub segments: Segments,
}

#[derive(Debug, Fail)]
pub enum ConfigError {
    #[fail(display = "serde_yaml::Error: {}", 0)]
    SerdeYamlError(serde_yaml::Error),
}

impl From<serde_yaml::Error> for ConfigError {
    fn from(err: serde_yaml::Error) -> Self {
        Self::SerdeYamlError(err)
    }
}

impl Config {
    pub fn config_path() -> String {
        let default_config_file_name = "almel.yaml";

        if let Ok(path) = env::var("ALMEL_CONFIG_FILE") {
            path
        } else if let Ok(config_home) = env::var("XDG_CONFIG_HOME") {
            format!("{}/almel/{}", config_home, default_config_file_name)
        } else if let Ok(home) = env::var("HOME") {
            format!("{}/.config/almel/{}", home, default_config_file_name)
        } else {
            format!("~/.config/almel/{}", default_config_file_name)
        }
    }

    pub fn load_from_str(string: &str) -> Result<Config, ConfigError> {
        let config = serde_yaml::from_str(string)?;

        Ok(config)
    }
}
