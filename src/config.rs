use failure::Fail;
use serde::{Deserialize, Serialize};

use crate::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct OsConfigEntry {
    pub background: String,
    pub foreground: String,
    pub icon: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsConfig {
    pub linux: OsConfigEntry,
    pub mac: OsConfigEntry,
    pub windows: OsConfigEntry,
}

impl OsConfig {
    #[cfg(target_os = "linux")]
    pub fn entry(&self) -> &OsConfigEntry {
        // TODO: Distributions
        &self.linux
    }

    #[cfg(target_os = "macos")]
    pub fn entry(&self) -> &OsConfigEntry {
        &self.mac
    }

    #[cfg(target_os = "windows")]
    pub fn entry(&self) -> &OsConfigEntry {
        &self.windows
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserConfig {
    pub background: String,
    pub foreground: String,
    pub display_host: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirConfig {
    pub background: String,
    pub foreground: String,
    pub shrink_path: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitUserConfig {
    pub display: bool,
    pub background: String,
    pub foreground: String,
    pub icon: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitConfig {
    pub user: GitUserConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewLineConfig {}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusConfigSucceeded {
    pub background: String,
    pub foreground: String,
    pub icon: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusConfigFailed {
    pub background: String,
    pub foreground: String,
    pub icon: String,
    pub display_exit_code: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusConfig {
    pub root_icon: String,
    pub job_icon: String,
    pub succeeded: StatusConfigSucceeded,
    pub failed: StatusConfigFailed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SegmentSeparators {
    pub left_solid: String,
}

type Segments = Vec<String>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub os: OsConfig,
    pub user: UserConfig,
    pub dir: DirConfig,
    pub git: GitConfig,
    pub newline: NewLineConfig,
    pub status: StatusConfig,
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
