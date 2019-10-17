use std::io;

use crate::env;
use crate::prompt::{Prompt, PromptError};

pub fn prompt_segment<W: io::Write>(p: &mut Prompt<W>) -> Result<(), PromptError> {
    let exit_status = env::var("exit_status")?.parse::<i32>().unwrap_or(-1);
    let uid = users::get_current_uid();
    let jobs = env::var("jobs")?;

    let success_icon = "\u{f62b}";
    let failure_icon = "\u{f06a}";
    let root_icon = "\u{e00a}";
    let jobs_icon = "\u{f013}";
    let display_exit_status_on_fail = true;

    let mut segment = String::new();
    let background;
    let foreground;

    // Exit status
    if exit_status == 0 {
        // Succeeded
        background = "white";
        foreground = "blue";

        segment += success_icon;
    } else {
        // Failed
        background = "red";
        foreground = "white";

        if display_exit_status_on_fail {
            segment += &format!("{} {}", failure_icon, exit_status);
        } else {
            segment += &format!("{}", failure_icon);
        }
    }

    // Root user?
    if uid == 0 {
        segment += " ";
        segment += root_icon;
    }

    // Jobs
    if !jobs.is_empty() {
        segment += " ";
        segment += jobs_icon;
    }

    p.write_segment(background, foreground, &segment)?;

    Ok(())
}
