use std::io;

use crate::config::NewLineConfig;
use crate::prompt::{Prompt, PromptError};

pub fn prompt_segment<W: io::Write>(
    p: &mut Prompt<W>,
    _config: &NewLineConfig,
) -> Result<(), PromptError> {
    p.close_segments()?;
    p.write("\n")?;

    Ok(())
}
