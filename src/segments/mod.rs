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

use crate::color::Color;
use crate::context::Context;

type SegmentError = Error;

#[derive(Debug)]
pub struct Segment {
    pub background: Color,
    pub foreground: Color,
    pub content: String,
}

pub fn build_segment(context: &Context, name: &str) -> Result<Option<Segment>, SegmentError> {
    match name {
        "os" => os::build_segment(&context),
        "shell" => shell::build_segment(&context),
        "directory" => directory::build_segment(&context),
        "user" => user::build_segment(&context),
        "status" => status::build_segment(&context),
        "time" => time::build_segment(&context),
        "duration" => duration::build_segment(&context),
        "git_repo" => git_repo::build_segment(&context),
        "git_user" => git_user::build_segment(&context),
        "venv" => venv::build_segment(&context),
        _ => Err(format_err!("Unknown segment: {}", name)),
    }
}
