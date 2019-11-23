use serde::{Deserialize, Serialize};
use std::default::Default;

use crate::color;
use crate::color::Color;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub linux: ConfigLinux,

    #[serde(default)]
    pub mac: ConfigMac,

    #[serde(default)]
    pub windows: ConfigWindows,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            linux: ConfigLinux::default(),
            mac: ConfigMac::default(),
            windows: ConfigWindows::default(),
        }
    }
}

// Linux
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigLinux {
    #[serde(default = "ConfigLinux::default_background")]
    pub background: Color,

    #[serde(default = "ConfigLinux::default_foreground")]
    pub foreground: Color,

    #[serde(default = "ConfigLinux::default_icon")]
    pub icon: String,
}

impl ConfigLinux {
    fn default_background() -> Color {
        color::CYAN
    }
    fn default_foreground() -> Color {
        color::WHITE
    }
    fn default_icon() -> String {
        "\u{f17c}".to_string() // nf-fa-linux
    }
}

impl Default for ConfigLinux {
    fn default() -> Self {
        Self {
            background: Self::default_background(),
            foreground: Self::default_foreground(),
            icon: Self::default_icon(),
        }
    }
}

// Mac
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigMac {
    #[serde(default = "ConfigMac::default_background")]
    pub background: Color,

    #[serde(default = "ConfigMac::default_foreground")]
    pub foreground: Color,

    #[serde(default = "ConfigMac::default_icon")]
    pub icon: String,
}

impl ConfigMac {
    fn default_background() -> Color {
        color::CYAN
    }
    fn default_foreground() -> Color {
        color::WHITE
    }
    fn default_icon() -> String {
        "\u{f179}".to_string() // nf-fa-apple
    }
}

impl Default for ConfigMac {
    fn default() -> Self {
        Self {
            background: Self::default_background(),
            foreground: Self::default_foreground(),
            icon: Self::default_icon(),
        }
    }
}

// Windows
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigWindows {
    #[serde(default = "ConfigWindows::default_background")]
    pub background: Color,

    #[serde(default = "ConfigWindows::default_foreground")]
    pub foreground: Color,

    #[serde(default = "ConfigWindows::default_icon")]
    pub icon: String,
}

impl ConfigWindows {
    fn default_background() -> Color {
        color::CYAN
    }
    fn default_foreground() -> Color {
        color::WHITE
    }
    fn default_icon() -> String {
        "\u{f17a}".to_string() // nf-fa-windows
    }
}

impl Default for ConfigWindows {
    fn default() -> Self {
        Self {
            background: Self::default_background(),
            foreground: Self::default_foreground(),
            icon: Self::default_icon(),
        }
    }
}
