use failure::Fail;
use std::io;

use crate::env;
use crate::glyph;
use crate::segments;
use crate::shell::Shell;

#[derive(Debug, Fail)]
pub enum PromptError {
    #[fail(display = "Unknown segment name: {}", 0)]
    UnknownSegment(String),

    #[fail(display = "IO Error: {}", 0)]
    IOError(io::Error),

    #[fail(display = "Env error: {}", 0)]
    EnvError(env::EnvError),
}

impl From<io::Error> for PromptError {
    fn from(err: io::Error) -> PromptError {
        PromptError::IOError(err)
    }
}

impl From<env::EnvError> for PromptError {
    fn from(err: env::EnvError) -> PromptError {
        PromptError::EnvError(err)
    }
}

pub struct Prompt<'w, W: io::Write> {
    pub shell: Shell,
    pub output: &'w mut W,
    pub current_bg: Option<String>,
}

impl<'w, W: io::Write> Prompt<'w, W> {
    pub fn new(shell: Shell, output: &'w mut W) -> Self {
        Self {
            shell,
            output,
            current_bg: None,
        }
    }

    fn set_color(&mut self, background: &str, foreground: &str) -> io::Result<()> {
        match self.shell {
            Shell::Zsh => write!(
                self.output,
                "%{{%K{{{}}}%F{{{}}}%}}",
                background, foreground
            ),
        }
    }

    fn write_segment_separator(&mut self, background: &str, foreground: &str) -> io::Result<()> {
        let current_bg = std::mem::replace(&mut self.current_bg, Some(background.to_string()));

        if let Some(current_bg) = current_bg {
            self.set_color(background, &current_bg)?;
            write!(self.output, "{}", glyph::segment_separator::LEFT_SOLID)?;
        }

        self.set_color(background, foreground)?;

        Ok(())
    }

    pub fn write_segment(
        &mut self,
        background: &str,
        foreground: &str,
        segment: &str,
    ) -> io::Result<()> {
        self.write_segment_separator(background, foreground)?;
        write!(self.output, " {} ", segment)?;

        Ok(())
    }

    pub fn close_segments(&mut self) -> io::Result<()> {
        self.write_segment_separator("default", "default")?;
        self.current_bg = None;

        write!(self.output, " ")?;

        Ok(())
    }
}

pub fn prompt(shell: Shell) -> Result<(), PromptError> {
    let mut buffer = std::io::stdout();
    let mut p = Prompt::new(shell, &mut buffer);
    let segments = ["user", "dir", "exit_status"];

    for segment in &segments {
        segments::prompt_segment(&mut p, segment)?;
    }

    p.close_segments()?;

    Ok(())
}
