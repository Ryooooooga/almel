use serde::{Deserialize, Serialize};
use std::default::Default;

use crate::configs::SegmentStyle;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub style: SegmentStyle,

    #[serde(default = "Config::default_format")]
    pub format: String,

    #[serde(default = "Config::default_utc")]
    pub utc: bool,
}
impl Config {
    fn default_format() -> String {
        // nf-fa-clock_o
        "\u{f017} %Y/%m/%d %H:%M:%S".to_string()
    }
    fn default_utc() -> bool {
        false
    }
}
impl Default for Config {
    fn default() -> Self {
        Self {
            style: Default::default(),
            format: Self::default_format(),
            utc: Self::default_utc(),
        }
    }
}
