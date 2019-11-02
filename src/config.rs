use failure::Error;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, File};
use std::io::prelude::Write; // File#write_all
use std::path::{Path, PathBuf};

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

// Directory
#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryConfig {
    pub normal: DirectoryConfigNormal,
    pub error: DirectoryConfigError,
    pub home: String,
    pub shrink: DirectoryConfigShrink,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryConfigNormal {
    pub background: Color,
    pub foreground: Color,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryConfigError {
    pub background: Color,
    pub foreground: Color,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryConfigShrink {
    pub enabled: bool,
    pub max_len: usize,
}

// Time
#[derive(Debug, Serialize, Deserialize)]
pub struct TimeConfig {
    pub background: Color,
    pub foreground: Color,
    pub icon: String,
    pub format: String,
    pub utc: bool,
}

// Git repository
#[derive(Debug, Serialize, Deserialize)]
pub struct GitRepoConfig {
    pub icons: GitRepoConfigIcons,
    pub clean: GitRepoConfigColors,
    pub unstaged: GitRepoConfigColors,
    pub staged: GitRepoConfigColors,
    pub conflicted: GitRepoConfigColors,
    pub display_tag: bool,
    pub commit_hash_len: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitRepoConfigIcons {
    pub branch: String,
    pub tag: String,
    pub commit: String,
    pub modified: String,
    pub added: String,
    pub deleted: String,
    pub added_deleted: String,
    pub conflicted: String,
    pub behind: String,
    pub ahead: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitRepoConfigColors {
    pub background: Color,
    pub foreground: Color,
}

// Git user
#[derive(Debug, Serialize, Deserialize)]
pub struct GitUserConfig {
    pub background: Color,
    pub foreground: Color,
    pub icon: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub os: OsConfig,
    pub user: UserConfig,
    pub directory: DirectoryConfig,
    pub git_repo: GitRepoConfig,
    pub git_user: GitUserConfig,
    pub status: StatusConfig,
    pub time: TimeConfig,
    pub segments: Vec<Vec<String>>,
}

type ConfigError = Error;

impl Config {
    pub fn load_from_str(s: &str) -> Result<Self, ConfigError> {
        let config = serde_yaml::from_str(s)?;

        Ok(config)
    }

    pub fn load_from_file(file: &File) -> Result<Self, ConfigError> {
        let config = serde_yaml::from_reader(file)?;

        Ok(config)
    }

    fn save_default_config<P: AsRef<Path>>(config_path: P) -> Result<(), ConfigError> {
        let config_path = config_path.as_ref();

        if let Some(config_dir) = config_path.parent() {
            create_dir_all(config_dir)?;
        }

        let mut config_file = File::create(config_path)?;
        config_file.write_all(DEFAULT_CONFIG_STR.as_bytes())?;

        Ok(())
    }

    pub fn load_from_file_or_create_default<P: AsRef<Path>>(
        config_path: P,
    ) -> Result<Config, ConfigError> {
        if let Ok(config_file) = File::open(&config_path) {
            let config = Self::load_from_file(&config_file)?;

            Ok(config)
        } else {
            // No config file
            let _ = Self::save_default_config(&config_path); // Ignore error
            let config = Self::load_from_str(&DEFAULT_CONFIG_STR)?;

            Ok(config)
        }
    }

    pub fn config_path() -> PathBuf {
        if let Ok(path) = std::env::var("ALMEL_CONFIG_PATH").map(PathBuf::from) {
            path
        } else if let Ok(config_home) = std::env::var("XDG_CONFIG_HOME").map(PathBuf::from) {
            let mut path = config_home;
            path.push("almel/almel.yaml");

            path
        } else if let Some(home) = dirs::home_dir() {
            let mut path = home;
            path.push(".config/almel/almel.yaml");

            path
        } else {
            PathBuf::from("~/.config/almel/almel.yaml")
        }
    }
}

lazy_static! {
    static ref DEFAULT_CONFIG_STR: &'static str = include_str!("almel.yaml");
}
