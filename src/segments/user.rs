use std::io;

use crate::config::UserConfig;
use crate::prompt::{Prompt, PromptError};

pub fn prompt_segment<W: io::Write>(
    p: &mut Prompt<W>,
    config: &UserConfig,
) -> Result<(), PromptError> {
    let text;

    if config.display_host {
        text = "%n@%m"
    } else {
        text = "%n"
    }

    p.write_segment(&config.background, &config.foreground, text)?;

    Ok(())
}
