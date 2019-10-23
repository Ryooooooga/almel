use failure::Error;
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::env;
use std::io;

use crate::prompt::Prompt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "Config::default_root_icon")]
    pub root_icon: String,

    #[serde(default = "Config::default_job_icon")]
    pub job_icon: String,

    #[serde(default)]
    pub succeeded: ConfigSucceeded,

    #[serde(default)]
    pub failed: ConfigFailed,
}

impl Config {
    fn default_root_icon() -> String {
        "\u{e00a}".to_string() // nf-pom-external_interruption
    }

    fn default_job_icon() -> String {
        "\u{f013}".to_string() // nf-fa-gear
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigSucceeded {
    #[serde(default = "ConfigSucceeded::default_background")]
    pub background: String,

    #[serde(default = "ConfigSucceeded::default_foreground")]
    pub foreground: String,

    #[serde(default = "ConfigSucceeded::default_icon")]
    pub icon: String,
}

impl ConfigSucceeded {
    fn default_background() -> String {
        "white".to_string()
    }

    fn default_foreground() -> String {
        "blue".to_string()
    }

    fn default_icon() -> String {
        "\u{f62b}".to_string() // nf-mdi-check
    }
}

impl Default for ConfigSucceeded {
    fn default() -> Self {
        Self {
            background: Self::default_background(),
            foreground: Self::default_foreground(),
            icon: Self::default_icon(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigFailed {
    #[serde(default = "ConfigFailed::default_background")]
    pub background: String,

    #[serde(default = "ConfigFailed::default_foreground")]
    pub foreground: String,

    #[serde(default = "ConfigFailed::default_icon")]
    pub icon: String,

    #[serde(default = "ConfigFailed::default_display_exit_code")]
    pub display_exit_code: bool,
}

impl ConfigFailed {
    fn default_background() -> String {
        "red".to_string()
    }

    fn default_foreground() -> String {
        "white".to_string()
    }

    fn default_icon() -> String {
        "\u{f06a}".to_string() // nf-fa-exclamation_circle
    }

    fn default_display_exit_code() -> bool {
        true
    }
}

impl Default for ConfigFailed {
    fn default() -> Self {
        Self {
            background: Self::default_background(),
            foreground: Self::default_foreground(),
            icon: Self::default_icon(),
            display_exit_code: Self::default_display_exit_code(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            root_icon: Self::default_root_icon(),
            job_icon: Self::default_job_icon(),
            succeeded: ConfigSucceeded::default(),
            failed: ConfigFailed::default(),
        }
    }
}

pub fn prompt_segment<W: io::Write>(p: &mut Prompt<W>, config: &Config) -> Result<(), Error> {
    let exit_status = env::var("exit_status")?.parse::<i32>().unwrap_or(-1);
    let uid = users::get_current_uid();
    let jobs = env::var("jobs")?;

    let mut segment = String::new();
    let background: &str;
    let foreground: &str;

    // Exit status
    if exit_status == 0 {
        // Succeeded
        background = &config.succeeded.background;
        foreground = &config.succeeded.foreground;

        segment += &config.succeeded.icon;
    } else {
        // Failed
        background = &config.failed.background;
        foreground = &config.failed.foreground;

        segment += &config.failed.icon;

        if config.failed.display_exit_code {
            segment += &format!(" {}", exit_status);
        }
    }

    // Root user?
    if uid == 0 {
        segment += " ";
        segment += &config.root_icon;
    }

    // Jobs
    if !jobs.is_empty() {
        segment += " ";
        segment += &config.job_icon;
    }

    p.write_segment(background, foreground, &segment)?;

    Ok(())
}
