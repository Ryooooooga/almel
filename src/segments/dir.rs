use dirs;
use failure::Error;
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::io;
use std::path::{Path, PathBuf};

use crate::prompt::Prompt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "Config::default_background")]
    pub background: String,

    #[serde(default = "Config::default_foreground")]
    pub foreground: String,

    #[serde(default = "Config::default_home")]
    pub home: String,

    #[serde(default)]
    pub shrink_path: ConfigShrinkPath,
}

impl Config {
    fn default_background() -> String {
        "blue".to_string()
    }

    fn default_foreground() -> String {
        "black".to_string()
    }

    fn default_home() -> String {
        "~".to_string()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            background: Self::default_background(),
            foreground: Self::default_foreground(),
            home: Self::default_home(),
            shrink_path: ConfigShrinkPath::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigShrinkPath {
    #[serde(default = "ConfigShrinkPath::default_enable")]
    pub enable: bool,

    #[serde(default = "ConfigShrinkPath::default_max_len")]
    pub max_len: usize,
}

impl ConfigShrinkPath {
    fn default_enable() -> bool {
        true
    }

    fn default_max_len() -> usize {
        1
    }
}

impl Default for ConfigShrinkPath {
    fn default() -> Self {
        Self {
            enable: Self::default_enable(),
            max_len: Self::default_max_len(),
        }
    }
}

fn shrink_path<P: AsRef<Path>>(out: &mut PathBuf, path: P, max_len: usize) {
    let mut iter = path.as_ref().iter();
    let last = iter.next_back();

    for s in iter {
        let s = s.to_str().unwrap_or("?");

        let max_len = if s.starts_with(".") {
            max_len + 1
        } else {
            max_len
        };

        out.push(s.chars().take(max_len).collect::<String>());
    }

    if let Some(last) = last {
        out.push(last);
    }
}

pub fn prompt_segment<W: io::Write>(p: &mut Prompt<W>, config: &Config) -> Result<(), Error> {
    let cd = std::env::current_dir()?;
    let mut cd = cd.as_path();

    let mut dir = PathBuf::new();

    // Replace $HOME
    let home = dirs::home_dir();

    if let Some(home) = home {
        if let Ok(postfix) = cd.strip_prefix(home) {
            cd = postfix;
            dir.push(&config.home);
        }
    }

    if config.shrink_path.enable {
        shrink_path(&mut dir, cd, config.shrink_path.max_len);
    } else {
        dir.push(cd);
    }

    p.write_segment(
        &config.background,
        &config.foreground,
        dir.to_str().unwrap_or("?"),
    )?;

    Ok(())
}
