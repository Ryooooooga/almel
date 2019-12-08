pub mod directory;
pub mod duration;
pub mod git_repo;
pub mod git_user;
pub mod os;
pub mod shell;
pub mod status;
pub mod time;
pub mod user;
pub mod venv;

use failure::Error;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::fs::{create_dir_all, File};
use std::io::prelude::Write; // File#write_all
use std::path::{Path, PathBuf};

// Separators
#[derive(Debug, Serialize, Deserialize)]
pub struct SegmentSeparators {
    #[serde(default = "SegmentSeparators::default_left_solid")]
    pub left_solid: String,
}
impl SegmentSeparators {
    fn default_left_solid() -> String {
        "\u{e0b0}".to_string() // nf-pl-left_hard_divider
    }
}
impl Default for SegmentSeparators {
    fn default() -> Self {
        Self {
            left_solid: Self::default_left_solid(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub os: os::Config,

    #[serde(default)]
    pub shell: shell::Config,

    #[serde(default)]
    pub user: user::Config,

    #[serde(default)]
    pub directory: directory::Config,

    #[serde(default)]
    pub git_repo: git_repo::Config,

    #[serde(default)]
    pub git_user: git_user::Config,

    #[serde(default)]
    pub status: status::Config,

    #[serde(default)]
    pub time: time::Config,

    #[serde(default)]
    pub duration: duration::Config,

    #[serde(default)]
    pub venv: venv::Config,

    #[serde(default)]
    pub segment_separators: SegmentSeparators,

    #[serde(default = "Config::default_segments")]
    pub segments: Vec<Vec<String>>,
}
impl Config {
    fn default_segments() -> Vec<Vec<String>> {
        [
            vec![
                "os",
                "shell",
                "time",
                "user",
                "directory",
                "git_repo",
                "git_user",
                "venv",
            ],
            vec!["duration", "status"],
        ]
        .iter()
        .map(|line| line.iter().map(|s| s.to_string()).collect())
        .collect()
    }
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
        if let Some(path) = std::env::var_os("ALMEL_CONFIG_PATH").map(PathBuf::from) {
            path
        } else if let Some(config_home) = std::env::var_os("XDG_CONFIG_HOME").map(PathBuf::from) {
            let mut path = config_home;
            path.push("almel/almel.yaml");

            path
        } else if let Some(home) = dirs::home_dir() {
            let mut path = home;
            path.push(".config/almel/almel.yaml");

            path
        } else {
            PathBuf::from("/etc/almel/almel.yaml")
        }
    }
}

lazy_static! {
    static ref DEFAULT_CONFIG_STR: &'static str = include_str!("almel.yaml");
}
