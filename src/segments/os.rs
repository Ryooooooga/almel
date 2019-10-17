use std::io;

use crate::prompt::{Prompt, PromptError};

#[cfg(target_os = "linux")]
fn get_os_icon() -> String {
    // TODO: Distributions
    "\u{f17c}".to_string()
}

#[cfg(target_os = "macos")]
fn get_os_icon() -> String {
    "\u{f179}".to_string()
}

#[cfg(target_os = "windows")]
fn get_os_icon() -> String {
    "\u{f17a}".to_string()
}

pub fn prompt_segment<W: io::Write>(p: &mut Prompt<W>) -> Result<(), PromptError> {
    p.write_segment("cyan", "white", &get_os_icon())?;

    Ok(())
}
