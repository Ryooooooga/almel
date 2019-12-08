use serde::{Deserialize, Serialize};
use std::default::Default;

use crate::configs::SegmentStyle;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub style: SegmentStyle,

    #[serde(default = "Config::default_icon")]
    pub icon: String,
}
impl Config {
    fn default_icon() -> String {
        "\u{f2c0}".to_string() // nf-fa-user_o
    }
}
impl Default for Config {
    fn default() -> Self {
        Self {
            style: Default::default(),
            icon: Self::default_icon(),
        }
    }
}
