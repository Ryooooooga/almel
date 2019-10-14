pub mod dir;
pub mod exit_status;
pub mod user;

use std::fmt;

use crate::prompt::{Prompt, PromptError};

pub fn prompt_segment<W: fmt::Write>(
    p: &mut Prompt<W>,
    segment_name: &str,
) -> Result<(), PromptError> {
    match segment_name {
        "user" => user::prompt_segment(p),
        "dir" => dir::prompt_segment(p),
        "exit_status" => exit_status::prompt_segment(p),
        _ => panic!("unknown segment '{}'", segment_name),
    }
}
