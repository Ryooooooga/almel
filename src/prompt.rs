use failure::Error;
use std::io;

use crate::config::{Config, SegmentSeparators};
use crate::segments;
use crate::shell::Shell;

pub struct Prompt<'w, W: io::Write> {
    pub shell: Shell,
    output: &'w mut W,
    current_bg: Option<String>,
    segment_separators: &'w SegmentSeparators,
}

impl<'w, W: io::Write> Prompt<'w, W> {
    pub fn new(shell: Shell, output: &'w mut W, segment_separators: &'w SegmentSeparators) -> Self {
        Self {
            shell,
            output,
            current_bg: None,
            segment_separators,
        }
    }

    pub fn write(&mut self, text: impl AsRef<str>) -> io::Result<()> {
        write!(self.output, "{}", text.as_ref())
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
            write!(self.output, "{}", self.segment_separators.left_solid)?;
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

        Ok(())
    }
}

pub fn prompt(shell: Shell) -> Result<(), Error> {
    let config = Config::load_from_file_or_create()?;

    let mut buffer = std::io::stdout();
    let mut p = Prompt::new(shell, &mut buffer, &config.segment_separators);

    for segment in &config.segments {
        segments::prompt_segment(&mut p, &config, segment)?;
    }

    p.close_segments()?;
    p.write(" ")?;

    Ok(())
}
