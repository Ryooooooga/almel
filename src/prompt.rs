use failure::Fail;
use std::fmt;

use crate::env;
use crate::segments;
use crate::shell::Shell;

#[derive(Debug, Fail)]
pub enum PromptError {
    #[fail(display = "Unknown segment name: {}", 0)]
    UnknownSegment(String),

    #[fail(display = "Format Error: {}", 0)]
    FmtError(fmt::Error),

    #[fail(display = "Env error: {}", 0)]
    EnvError(env::EnvError),
}

impl From<fmt::Error> for PromptError {
    fn from(err: fmt::Error) -> PromptError {
        PromptError::FmtError(err)
    }
}

impl From<env::EnvError> for PromptError {
    fn from(err: env::EnvError) -> PromptError {
        PromptError::EnvError(err)
    }
}

pub struct Prompt<'w, W: fmt::Write> {
    pub shell: Shell,
    pub output: &'w mut W,
    pub current_bg: Option<String>,
}

impl<'w, W: fmt::Write> Prompt<'w, W> {
    pub fn new(shell: Shell, output: &'w mut W) -> Self {
        Self {
            shell,
            output,
            current_bg: None,
        }
    }

    pub fn write_segment(
        &mut self,
        background: &str,
        foreground: &str,
        segment: &str,
    ) -> fmt::Result {
        write!(self.output, " {} ", segment)
    }
}

pub fn prompt(shell: Shell) -> Result<(), PromptError> {
    let mut buffer = String::new();
    let mut p = Prompt::new(shell, &mut buffer);
    let segments = ["user", "dir", "exit_status"];

    for segment in &segments {
        segments::prompt_segment(&mut p, segment)?;
    }

    print!("{}", buffer);

    Ok(())
}
