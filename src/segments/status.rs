use std::io;

use crate::config::StatusConfig;
use crate::env;
use crate::prompt::{Prompt, PromptError};

pub fn prompt_segment<W: io::Write>(
    p: &mut Prompt<W>,
    config: &StatusConfig,
) -> Result<(), PromptError> {
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
