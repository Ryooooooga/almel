use std::io;

use crate::prompt::{Prompt, PromptError};

pub fn prompt_segment<W: io::Write>(p: &mut Prompt<W>) -> Result<(), PromptError> {
    p.close_segments()?;
    p.write("\n")?;

    Ok(())
}
