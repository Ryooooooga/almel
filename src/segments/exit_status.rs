use std::fmt;

use crate::env;
use crate::prompt::{Prompt, PromptError};

pub fn prompt_segment<W: fmt::Write>(p: &mut Prompt<W>) -> Result<(), PromptError> {
    let exit_status = env::var("exit_status")?.parse::<i32>().unwrap_or(-1);

    p.write_segment("white", "lime", &format!("{}", exit_status))?;

    Ok(())
}
