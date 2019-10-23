mod head_status;
mod user;

use failure::Error;
use git2::Repository;
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::io;

use crate::prompt::Prompt;

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

pub fn prompt_segment<W: io::Write>(p: &mut Prompt<W>, config: &Config) -> Result<(), Error> {
    // Open the current repository
    if let Ok(repo) = Repository::discover(".") {
        head_status::prompt_subsegment(p, &config.head_status, &repo)?;
        user::prompt_subsegment(p, &config.user, &repo)?;
    }

    Ok(())
}
