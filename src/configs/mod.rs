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

use ansi_term::Color;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::fs::{create_dir_all, File};
use std::io::prelude::Write; // File#write_all
use std::path::{Path, PathBuf};

// SegmentStyle
#[derive(Debug, Serialize, Deserialize)]
pub struct SegmentStyle {
    #[serde(default = "SegmentStyle::default_foreground")]
    pub foreground: Color,

    #[serde(default = "SegmentStyle::default_background")]
    pub background: Color,
}
impl SegmentStyle {
    fn default_foreground() -> Color {
        Color::White
    }
    fn default_background() -> Color {
        Color::Black
    }
}
impl Default for SegmentStyle {
    fn default() -> Self {
        Self {
            foreground: Self::default_foreground(),
            background: Self::default_background(),
        }
    }
}

// Separators
#[derive(Debug, Serialize, Deserialize)]
pub struct SegmentSeparators {
    #[serde(default = "SegmentSeparators::default_left_solid")]
    pub left_solid: String,

    #[serde(default = "SegmentSeparators::default_left_wire")]
    pub left_wire: String,
}
impl SegmentSeparators {
    fn default_left_solid() -> String {
        "\u{e0b0}".to_string() // nf-pl-left_hard_divider
    }
    fn default_left_wire() -> String {
        "\u{e0b1}".to_string() // nf-pl-left_soft_divider
    }
}
impl Default for SegmentSeparators {
    fn default() -> Self {
        Self {
            left_solid: Self::default_left_solid(),
            left_wire: Self::default_left_wire(),
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
        vec![
            vec![
                "os".to_string(),
                "shell".to_string(),
                "time".to_string(),
                "user".to_string(),
                "directory".to_string(),
                "git_repo".to_string(),
                "git_user".to_string(),
                "venv".to_string(),
            ],
            vec!["duration".to_string(), "status".to_string()],
        ]
    }
}
impl Default for Config {
    fn default() -> Self {
        serde_yaml::from_str(DEFAULT_CONFIG_STR).unwrap()
    }
}

impl Config {
    pub fn load_from_str(s: &str) -> Result<Self> {
        let config = serde_yaml::from_str(s)?;

        Ok(config)
    }

    pub fn load_from_file(file: &File) -> Result<Self> {
        let config = serde_yaml::from_reader(file)?;

        Ok(config)
    }

    fn save_default_config<P: AsRef<Path>>(config_path: P) -> Result<()> {
        let config_path = config_path.as_ref();

        if let Some(config_dir) = config_path.parent() {
            create_dir_all(config_dir)?;
        }

        let mut config_file = File::create(config_path)?;
        config_file.write_all(DEFAULT_CONFIG_STR.as_bytes())?;

        Ok(())
    }

    pub fn load_from_file_or_create_default<P: AsRef<Path>>(config_path: P) -> Result<Config> {
        if let Ok(config_file) = File::open(&config_path) {
            let config = Self::load_from_file(&config_file)?;

            Ok(config)
        } else {
            // No config file
            let _ = Self::save_default_config(&config_path); // Ignore error
            let config = Self::load_from_str(DEFAULT_CONFIG_STR)?;

            Ok(config)
        }
    }

    pub fn config_path() -> PathBuf {
        if let Some(path) = std::env::var_os("ALMEL_CONFIG_FILE").map(PathBuf::from) {
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

const DEFAULT_CONFIG_STR: &str = include_str!("almel.yaml");
