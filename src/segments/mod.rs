mod directory;
mod duration;
mod git_repo;
mod git_user;
mod os;
mod shell;
mod status;
mod time;
mod user;
mod venv;

use failure::{format_err, Error};

use crate::configs::SegmentStyle;
use crate::context::Context;

#[derive(Debug)]
pub struct Segment<'a> {
    pub style: &'a SegmentStyle,
    pub content: String,
}

pub fn build_segment<'ctx>(
    context: &'ctx Context,
    name: &str,
) -> Result<Option<Segment<'ctx>>, Error> {
    match name {
        "os" => Ok(os::build_segment(&context)),
        "shell" => Ok(shell::build_segment(&context)),
        "directory" => Ok(directory::build_segment(&context)),
        "user" => Ok(user::build_segment(&context)),
        "status" => Ok(status::build_segment(&context)),
        "time" => Ok(time::build_segment(&context)),
        "duration" => Ok(duration::build_segment(&context)),
        "git_repo" => Ok(git_repo::build_segment(&context)),
        "git_user" => Ok(git_user::build_segment(&context)),
        "venv" => Ok(venv::build_segment(&context)),
        _ => Err(format_err!("Unknown segment: {}", name)),
    }
}
