mod head_status;
mod user;

use git2::Repository;
use std::io;

use crate::config::GitConfig;
use crate::prompt::{Prompt, PromptError};

pub fn prompt_segment<W: io::Write>(
    p: &mut Prompt<W>,
    config: &GitConfig,
) -> Result<(), PromptError> {
    // Open the current repository
    let repo = match Repository::discover(".") {
        Ok(repo) => repo,
        Err(_) => return Ok(()),
    };

    head_status::prompt_subsegment(p, &config.head_status, &repo)?;
    user::prompt_subsegment(p, &config.user, &repo)?;

    Ok(())
}
