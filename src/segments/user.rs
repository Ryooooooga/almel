use std::fmt;

use crate::prompt::{Prompt, PromptError};

pub fn prompt_segment<W: fmt::Write>(p: &mut Prompt<W>) -> Result<(), PromptError> {
    p.write_segment("black", "white", "%n@%m")?;

    Ok(())
}
