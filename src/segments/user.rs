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

    #[serde(default = "Config::default_display_host")]
    pub display_host: bool,
}

impl Config {
    fn default_background() -> String {
        "black".to_string()
    }

    fn default_foreground() -> String {
        "white".to_string()
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

pub fn prompt_segment<W: io::Write>(p: &mut Prompt<W>, config: &Config) -> Result<(), PromptError> {
    let text;

    if config.display_host {
        text = "%n@%m"
    } else {
        text = "%n"
    }

    p.write_segment(&config.background, &config.foreground, text)?;

    Ok(())
}
