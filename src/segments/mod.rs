pub mod dir;
pub mod git;
pub mod newline;
pub mod os;
pub mod status;
pub mod user;

use std::io;

use crate::config::Config;
use crate::prompt::{Prompt, PromptError};

pub fn prompt_segment<W: io::Write>(
    p: &mut Prompt<W>,
    config: &Config,
    segment_name: &str,
) -> Result<(), PromptError> {
    match segment_name {
        "os" => os::prompt_segment(p, &config.os),
        "user" => user::prompt_segment(p, &config.user),
        "dir" => dir::prompt_segment(p, &config.dir),
        "git" => git::prompt_segment(p, &config.git),
        "status" => status::prompt_segment(p, &config.status),
        "newline" => newline::prompt_segment(p, &config.newline),
        _ => panic!("unknown segment '{}'", segment_name),
    }
}
