use serde::{Deserialize, Serialize};
use std::default::Default;

use crate::color;
use crate::color::Color;

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
            normal: ConfigNormal::default(),
            error: ConfigError::default(),
            home: Self::default_home(),
            shrink: ConfigShrink::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigNormal {
    #[serde(default = "ConfigNormal::default_background")]
    pub background: Color,

    #[serde(default = "ConfigNormal::default_foreground")]
    pub foreground: Color,
}
impl ConfigNormal {
    fn default_background() -> Color {
        color::BLUE
    }
    fn default_foreground() -> Color {
        color::BLACK
    }
}
impl Default for ConfigNormal {
    fn default() -> Self {
        Self {
            background: Self::default_background(),
            foreground: Self::default_foreground(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigError {
    #[serde(default = "ConfigError::default_background")]
    pub background: Color,

    #[serde(default = "ConfigError::default_foreground")]
    pub foreground: Color,
}
impl ConfigError {
    fn default_background() -> Color {
        color::RED
    }
    fn default_foreground() -> Color {
        color::BLACK
    }
}
impl Default for ConfigError {
    fn default() -> Self {
        Self {
            background: Self::default_background(),
            foreground: Self::default_foreground(),
        }
    }
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
