use serde::{Deserialize, Serialize};
use std::default::Default;

use crate::configs::SegmentStyle;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub linux: ConfigLinux,

    #[serde(default)]
    pub mac: ConfigMac,

    #[serde(default)]
    pub windows: ConfigWindows,
}

// Linux
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigLinux {
    #[serde(default)]
    pub style: SegmentStyle,

    #[serde(default = "ConfigLinux::default_icon")]
    pub icon: String,
}

impl ConfigLinux {
    fn default_icon() -> String {
        "\u{f17c}".to_string() // nf-fa-linux
    }
}

impl Default for ConfigLinux {
    fn default() -> Self {
        Self {
            style: Default::default(),
            icon: Self::default_icon(),
        }
    }
}

// Mac
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigMac {
    #[serde(default)]
    pub style: SegmentStyle,

    #[serde(default = "ConfigMac::default_icon")]
    pub icon: String,
}

impl ConfigMac {
    fn default_icon() -> String {
        "\u{f179}".to_string() // nf-fa-apple
    }
}

impl Default for ConfigMac {
    fn default() -> Self {
        Self {
            style: Default::default(),
            icon: Self::default_icon(),
        }
    }
}

// Windows
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigWindows {
    #[serde(default)]
    pub style: SegmentStyle,

    #[serde(default = "ConfigWindows::default_icon")]
    pub icon: String,
}

impl ConfigWindows {
    fn default_icon() -> String {
        "\u{f17a}".to_string() // nf-fa-windows
    }
}

impl Default for ConfigWindows {
    fn default() -> Self {
        Self {
            style: Default::default(),
            icon: Self::default_icon(),
        }
    }
}
