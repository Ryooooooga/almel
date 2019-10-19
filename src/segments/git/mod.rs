mod head_status;
mod user;

use git2::Repository;
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::io;

use crate::prompt::{Prompt, PromptError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub head_status: head_status::Config,

    #[serde(default)]
    pub user: user::Config,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            head_status: head_status::Config::default(),
            user: user::Config::default(),
        }
    }
}

pub fn prompt_segment<W: io::Write>(p: &mut Prompt<W>, config: &Config) -> Result<(), PromptError> {
    // Open the current repository
    let repo = match Repository::discover(".") {
        Ok(repo) => repo,
        Err(_) => return Ok(()),
    };

    head_status::prompt_subsegment(p, &config.head_status, &repo)?;
    user::prompt_subsegment(p, &config.user, &repo)?;

    Ok(())
}
