use serde::{Deserialize, Serialize};
use std::default::Default;

use crate::configs::SegmentStyle;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub style: SegmentStyle,

    #[serde(default = "Config::default_display_host")]
    pub display_host: bool,
}

impl Config {
    fn default_display_host() -> bool {
        true
    }
}
impl Default for Config {
    fn default() -> Self {
        Self {
            style: Default::default(),
            display_host: Self::default_display_host(),
        }
    }
}
