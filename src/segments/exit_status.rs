use std::io;

use crate::env;
use crate::prompt::{Prompt, PromptError};

pub fn prompt_segment<W: io::Write>(p: &mut Prompt<W>) -> Result<(), PromptError> {
    let exit_status = env::var("exit_status")?.parse::<i32>().unwrap_or(-1);

    if exit_status == 0 {
        p.write_segment("white", "green", &format!("{}", exit_status))?;
    } else {
        p.write_segment("white", "red", &format!("{}", exit_status))?;
    }

    Ok(())
}
