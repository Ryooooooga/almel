use failure::Error;
use git2::Repository;
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::io::Write;

use crate::prompt::Prompt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "Config::default_display")]
    pub display: bool,

    #[serde(default = "Config::default_background")]
    pub background: String,

    #[serde(default = "Config::default_foreground")]
    pub foreground: String,

    #[serde(default = "Config::default_icon")]
    pub icon: String,
}

impl Config {
    fn default_display() -> bool {
        true
    }

    fn default_background() -> String {
        "cyan".to_string()
    }

    fn default_foreground() -> String {
        "black".to_string()
    }

    fn default_icon() -> String {
        "\u{f2c0}".to_string() // nf-fa-user_o
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            display: Self::default_display(),
            background: Self::default_background(),
            foreground: Self::default_foreground(),
            icon: Self::default_icon(),
        }
    }
}

pub fn prompt_subsegment<W: Write>(
    p: &mut Prompt<W>,
    config: &Config,
    repo: &Repository,
) -> Result<(), Error> {
    if !config.display {
        return Ok(());
    }

    if let Ok(cfg) = repo.config() {
        if let Ok(user_name) = cfg.get_string("user.name") {
            p.write_segment(
                &config.background,
                &config.foreground,
                &format!("{} {}", config.icon, user_name),
            )?;
        }
    }
    Ok(())
}
