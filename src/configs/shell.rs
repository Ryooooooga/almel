use serde::{Deserialize, Serialize};
use std::default::Default;

use crate::color;
use crate::color::Color;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub bash: ConfigBash,

    #[serde(default)]
    pub zsh: ConfigZsh,

    #[serde(default)]
    pub fish: ConfigFish,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            bash: ConfigBash::default(),
            zsh: ConfigZsh::default(),
            fish: ConfigFish::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigBash {
    #[serde(default = "ConfigBash::default_background")]
    pub background: Color,

    #[serde(default = "ConfigBash::default_foreground")]
    pub foreground: Color,

    #[serde(default = "ConfigBash::default_icon")]
    pub icon: String,
}
impl ConfigBash {
    fn default_background() -> Color {
        color::WHITE
    }
    fn default_foreground() -> Color {
        color::GREEN
    }
    fn default_icon() -> String {
        "Bash".to_string()
    }
}
impl Default for ConfigBash {
    fn default() -> Self {
        Self {
            background: Self::default_background(),
            foreground: Self::default_foreground(),
            icon: Self::default_icon(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigZsh {
    #[serde(default = "ConfigZsh::default_background")]
    pub background: Color,

    #[serde(default = "ConfigZsh::default_foreground")]
    pub foreground: Color,

    #[serde(default = "ConfigZsh::default_icon")]
    pub icon: String,
}
impl ConfigZsh {
    fn default_background() -> Color {
        color::WHITE
    }
    fn default_foreground() -> Color {
        color::GREEN
    }
    fn default_icon() -> String {
        "Zsh".to_string()
    }
}
impl Default for ConfigZsh {
    fn default() -> Self {
        Self {
            background: Self::default_background(),
            foreground: Self::default_foreground(),
            icon: Self::default_icon(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigFish {
    #[serde(default = "ConfigFish::default_background")]
    pub background: Color,

    #[serde(default = "ConfigFish::default_foreground")]
    pub foreground: Color,

    #[serde(default = "ConfigFish::default_icon")]
    pub icon: String,
}
impl ConfigFish {
    fn default_background() -> Color {
        color::WHITE
    }
    fn default_foreground() -> Color {
        color::GREEN
    }
    fn default_icon() -> String {
        "\u{f739}".to_string() // nf-mdi-fish
    }
}
impl Default for ConfigFish {
    fn default() -> Self {
        Self {
            background: Self::default_background(),
            foreground: Self::default_foreground(),
            icon: Self::default_icon(),
        }
    }
}
