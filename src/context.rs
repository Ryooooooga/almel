use failure::Error;
use git2::Repository;
use std::path::PathBuf;

use crate::configs::Config;
use crate::opt::PromptArgs;

pub struct Context<'ctx> {
    pub current_dir: PathBuf,
    pub config: &'ctx Config,
    pub opt: &'ctx PromptArgs,
    pub git_repo: Option<Repository>,
}

impl<'ctx> Context<'ctx> {
    pub fn new(opt: &'ctx PromptArgs, config: &'ctx Config) -> Result<Self, Error> {
        let current_dir = std::env::var_os("PWD")
            .map(PathBuf::from)
            .or_else(|| std::env::current_dir().ok())
            .expect("Could not estimate current directory");

        let git_repo = Repository::discover(&current_dir).ok();

        Ok(Self {
            current_dir,
            config,
            opt,
            git_repo,
        })
    }
}
