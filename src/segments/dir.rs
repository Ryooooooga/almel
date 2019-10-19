use std::io;

use crate::config::DirConfig;
use crate::prompt::{Prompt, PromptError};

pub fn prompt_segment<W: io::Write>(
    p: &mut Prompt<W>,
    config: &DirConfig,
) -> Result<(), PromptError> {
    let cd = std::env::current_dir()?;

    p.write_segment(
        &config.background,
        &config.foreground,
        cd.to_str().unwrap_or("?"),
    )?;

    Ok(())
}
