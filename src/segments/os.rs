use failure::Error;
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::io;

use crate::prompt::Prompt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "ConfigEntry::default_linux")]
    pub linux: ConfigEntry,

    #[serde(default = "ConfigEntry::default_mac")]
    pub mac: ConfigEntry,

    #[serde(default = "ConfigEntry::default_windows")]
    pub windows: ConfigEntry,
}

impl Config {
    #[cfg(target_os = "linux")]
    fn entry(&self) -> &ConfigEntry {
        // TODO: Distributions
        &self.linux
    }

    #[cfg(target_os = "macos")]
    fn entry(&self) -> &ConfigEntry {
        &self.mac
    }

    #[cfg(target_os = "windows")]
    fn entry(&self) -> &ConfigEntry {
        &self.windows
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            linux: ConfigEntry::default_linux(),
            mac: ConfigEntry::default_mac(),
            windows: ConfigEntry::default_windows(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigEntry {
    #[serde(default = "ConfigEntry::default_background")]
    pub background: String,

    #[serde(default = "ConfigEntry::default_foreground")]
    pub foreground: String,

    pub icon: String,
}

impl ConfigEntry {
    fn default_background() -> String {
        "cyan".to_string()
    }

    fn default_foreground() -> String {
        "white".to_string()
    }

    fn default_linux() -> Self {
        Self {
            background: Self::default_background(),
            foreground: Self::default_foreground(),
            icon: "\u{f17c}".to_string(), // nf-fa-linux
        }
    }

    fn default_mac() -> Self {
        Self {
            background: Self::default_background(),
            foreground: Self::default_foreground(),
            icon: "\u{f179}".to_string(), // nf-fa-apple
        }
    }

    fn default_windows() -> Self {
        Self {
            background: Self::default_background(),
            foreground: Self::default_foreground(),
            icon: "\u{f17a}".to_string(), // nf-fa-windows
        }
    }
}

pub fn prompt_segment<W: io::Write>(p: &mut Prompt<W>, config: &Config) -> Result<(), Error> {
    let entry = config.entry();

    p.write_segment(&entry.background, &entry.foreground, &entry.icon)?;

    Ok(())
}
