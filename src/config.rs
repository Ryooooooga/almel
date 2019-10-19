use failure::Fail;
use serde::{Deserialize, Serialize};

use crate::env;
use crate::segments::dir;
use crate::segments::git;
use crate::segments::newline;
use crate::segments::os;
use crate::segments::status;
use crate::segments::user;

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

type Segments = Vec<String>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub os: os::Config,

    #[serde(default)]
    pub user: user::Config,

    #[serde(default)]
    pub dir: dir::Config,

    #[serde(default)]
    pub git: git::Config,

    #[serde(default)]
    pub newline: newline::Config,

    #[serde(default)]
    pub status: status::Config,

    #[serde(default)]
    pub segment_separators: SegmentSeparators,

    #[serde(default = "Config::default_segments")]
    pub segments: Segments,
}

impl Config {
    fn default_segments() -> Segments {
        ["os", "user", "dir", "git", "newline", "status"]
            .iter()
            .map(|s| s.to_string())
            .collect()
    }
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
