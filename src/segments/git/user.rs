use git2::Repository;
use std::io::Write;

use crate::config::GitUserConfig;
use crate::prompt::{Prompt, PromptError};

pub fn prompt_subsegment<W: Write>(
    p: &mut Prompt<W>,
    config: &GitUserConfig,
    repo: &Repository,
) -> Result<(), PromptError> {
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
