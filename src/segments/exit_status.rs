use std::fmt;

use crate::prompt::{Prompt, PromptError};

pub fn prompt_segment<W: fmt::Write>(p: &mut Prompt<W>) -> Result<(), PromptError> {
    let exit_status = std::env::var("exit_status")
        .ok()
        .and_then(|status| status.parse::<i32>().ok())
        .unwrap_or(-1);

    p.write_segment("white", "lime", &format!("{}", exit_status))?;

    Ok(())
}
