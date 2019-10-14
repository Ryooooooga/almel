use std::io;

use crate::prompt::{Prompt, PromptError};

pub fn prompt_segment<W: io::Write>(p: &mut Prompt<W>) -> Result<(), PromptError> {
    p.write_segment("blue", "black", "%~")?;

    Ok(())
}
