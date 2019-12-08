use serde::{Deserialize, Serialize};
use std::default::Default;

use crate::configs::SegmentStyle;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub normal: ConfigNormal,

    #[serde(default)]
    pub error: ConfigError,

    #[serde(default = "Config::default_home")]
    pub home: String,

    #[serde(default)]
    pub shrink: ConfigShrink,
}
impl Config {
    fn default_home() -> String {
        "~".to_string()
    }
}
impl Default for Config {
    fn default() -> Self {
        Self {
            normal: Default::default(),
            error: Default::default(),
            home: Self::default_home(),
            shrink: Default::default(),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ConfigNormal {
    #[serde(default)]
    pub style: SegmentStyle,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ConfigError {
    #[serde(default)]
    pub style: SegmentStyle,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigShrink {
    #[serde(default = "ConfigShrink::default_enabled")]
    pub enabled: bool,

    #[serde(default = "ConfigShrink::default_max_len")]
    pub max_len: usize,
}
impl ConfigShrink {
    fn default_enabled() -> bool {
        true
    }
    fn default_max_len() -> usize {
        1
    }
}
impl Default for ConfigShrink {
    fn default() -> Self {
        Self {
            enabled: Self::default_enabled(),
            max_len: Self::default_max_len(),
        }
    }
}
