use failure::Error;
use std::path::PathBuf;

use crate::config::{Config, DEFAULT_CONFIG_STR};
use crate::opt::PromptArgs;

#[derive(Debug)]
pub struct Context<'ctx> {
    pub current_dir: PathBuf,
    pub config: Config,
    pub opt: &'ctx PromptArgs,
}

impl<'ctx> Context<'ctx> {
    pub fn new(opt: &'ctx PromptArgs) -> Result<Self, Error> {
        let current_dir = std::env::current_dir()
            .ok()
            .or_else(|| std::env::var("PWD").ok().map(PathBuf::from))
            .expect("Could not estimate current directory");

        let config = Config::load_from_str(&DEFAULT_CONFIG_STR)?;

        Ok(Self {
            current_dir,
            config,
            opt,
        })
    }
}
