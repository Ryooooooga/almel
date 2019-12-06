use ansi_term::Color;
use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "Config::default_background")]
    pub background: Color,

    #[serde(default = "Config::default_foreground")]
    pub foreground: Color,

    #[serde(default = "Config::default_display_host")]
    pub display_host: bool,
}

impl Config {
    fn default_background() -> Color {
        Color::Black
    }
    fn default_foreground() -> Color {
        Color::White
    }
    fn default_display_host() -> bool {
        true
    }
}
impl Default for Config {
    fn default() -> Self {
        Self {
            background: Self::default_background(),
            foreground: Self::default_foreground(),
            display_host: Self::default_display_host(),
        }
    }
}
