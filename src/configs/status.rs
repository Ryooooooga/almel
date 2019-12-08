use serde::{Deserialize, Serialize};
use std::default::Default;

use crate::configs::SegmentStyle;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub icons: ConfigIcons,

    #[serde(default)]
    pub succeeded: ConfigSucceeded,

    #[serde(default)]
    pub failed: ConfigFailed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigIcons {
    #[serde(default = "ConfigIcons::default_succeeded")]
    pub succeeded: String,

    #[serde(default = "ConfigIcons::default_failed")]
    pub failed: String,

    #[serde(default = "ConfigIcons::default_root")]
    pub root: String,

    #[serde(default = "ConfigIcons::default_jobs")]
    pub jobs: String,
}
impl ConfigIcons {
    fn default_succeeded() -> String {
        "✓".to_string()
    }
    fn default_failed() -> String {
        "\u{f06a}".to_string() // nf-fa-exclamation_circle
    }
    fn default_root() -> String {
        "\u{e00a}".to_string() // nf-pom-external_interruption
    }
    fn default_jobs() -> String {
        "\u{f013}".to_string() // nf-fa-gear
    }
}
impl Default for ConfigIcons {
    fn default() -> Self {
        Self {
            succeeded: Self::default_succeeded(),
            failed: Self::default_failed(),
            root: Self::default_root(),
            jobs: Self::default_jobs(),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ConfigSucceeded {
    #[serde(default)]
    pub style: SegmentStyle,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigFailed {
    #[serde(default)]
    pub style: SegmentStyle,

    #[serde(default = "ConfigFailed::default_display_exit_status")]
    pub display_exit_status: bool,
}
impl ConfigFailed {
    fn default_display_exit_status() -> bool {
        true
    }
}
impl Default for ConfigFailed {
    fn default() -> Self {
        Self {
            style: Default::default(),
            display_exit_status: Self::default_display_exit_status(),
        }
    }
}
