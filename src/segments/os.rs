use std::io;

use crate::config::OsConfig;
use crate::prompt::{Prompt, PromptError};

pub fn prompt_segment<W: io::Write>(
    p: &mut Prompt<W>,
    config: &OsConfig,
) -> Result<(), PromptError> {
    let entry = config.entry();

    p.write_segment(&entry.background, &entry.foreground, &entry.icon)?;

    Ok(())
}
