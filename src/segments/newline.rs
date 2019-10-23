use failure::Error;
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::io;

use crate::prompt::Prompt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {}

impl Default for Config {
    fn default() -> Self {
        Self {}
    }
}

pub fn prompt_segment<W: io::Write>(p: &mut Prompt<W>, _config: &Config) -> Result<(), Error> {
    p.close_segments()?;
    p.write("\n")?;

    Ok(())
}
