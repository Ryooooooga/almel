use serde::{Deserialize, Serialize};
use std::default::Default;
use std::io;

use crate::prompt::{Prompt, PromptError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "Config::default_background")]
    pub background: String,

    #[serde(default = "Config::default_foreground")]
    pub foreground: String,

    #[serde(default = "Config::default_shrink_path")]
    pub shrink_path: bool,
}

impl Config {
    fn default_background() -> String {
        "blue".to_string()
    }

    fn default_foreground() -> String {
        "black".to_string()
    }

    fn default_shrink_path() -> bool {
        true
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            background: Self::default_background(),
            foreground: Self::default_foreground(),
            shrink_path: Self::default_shrink_path(),
        }
    }
}

pub fn prompt_segment<W: io::Write>(p: &mut Prompt<W>, config: &Config) -> Result<(), PromptError> {
    let cd = std::env::current_dir()?;

    p.write_segment(
        &config.background,
        &config.foreground,
        cd.to_str().unwrap_or("?"),
    )?;

    Ok(())
}
