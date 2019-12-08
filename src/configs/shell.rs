use serde::{Deserialize, Serialize};
use std::default::Default;

use crate::configs::SegmentStyle;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub bash: ConfigBash,

    #[serde(default)]
    pub zsh: ConfigZsh,

    #[serde(default)]
    pub fish: ConfigFish,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigBash {
    #[serde(default)]
    pub style: SegmentStyle,

    #[serde(default = "ConfigBash::default_icon")]
    pub icon: String,
}
impl ConfigBash {
    fn default_icon() -> String {
        "Bash".to_string()
    }
}
impl Default for ConfigBash {
    fn default() -> Self {
        Self {
            style: Default::default(),
            icon: Self::default_icon(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigZsh {
    #[serde(default)]
    pub style: SegmentStyle,

    #[serde(default = "ConfigZsh::default_icon")]
    pub icon: String,
}
impl ConfigZsh {
    fn default_icon() -> String {
        "Zsh".to_string()
    }
}
impl Default for ConfigZsh {
    fn default() -> Self {
        Self {
            style: Default::default(),
            icon: Self::default_icon(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigFish {
    #[serde(default)]
    pub style: SegmentStyle,

    #[serde(default = "ConfigFish::default_icon")]
    pub icon: String,
}
impl ConfigFish {
    fn default_icon() -> String {
        "\u{f739}".to_string() // nf-mdi-fish
    }
}
impl Default for ConfigFish {
    fn default() -> Self {
        Self {
            style: Default::default(),
            icon: Self::default_icon(),
        }
    }
}
