use std::path::PathBuf;

use crate::opt::PromptArgs;

#[derive(Debug)]
pub struct Context<'ctx> {
    current_dir: PathBuf,
    opt: &'ctx PromptArgs,
}

impl<'ctx> Context<'ctx> {
    pub fn new(opt: &'ctx PromptArgs) -> Self {
        let current_dir = std::env::current_dir()
            .ok()
            .or_else(|| std::env::var("PWD").ok().map(PathBuf::from))
            .expect("Could not estimate current directory");

        Self { current_dir, opt }
    }

    pub fn get_current_dir(&self) -> &PathBuf {
        &self.current_dir
    }

    pub fn get_exit_status(&self) -> i32 {
        self.opt.exit_status
    }

    pub fn get_num_jobs(&self) -> i32 {
        self.opt.num_jobs
    }
}
