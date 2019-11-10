use serde::{Deserialize, Serialize};
use std::default::Default;

use crate::color;
use crate::color::Color;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "Config::default_background")]
    pub background: Color,

    #[serde(default = "Config::default_foreground")]
    pub foreground: Color,

    #[serde(default = "Config::default_icon")]
    pub icon: String,
}
impl Config {
    fn default_background() -> Color {
        color::CYAN
    }
    fn default_foreground() -> Color {
        color::BLACK
    }
    fn default_icon() -> String {
        "\u{f2c0}".to_string() // nf-fa-user_o
    }
}
impl Default for Config {
    fn default() -> Self {
        Self {
            background: Self::default_background(),
            foreground: Self::default_foreground(),
            icon: Self::default_icon(),
        }
    }
}
