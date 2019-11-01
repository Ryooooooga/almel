mod directory;
mod git_user;
mod os;
mod status;
mod user;

use failure::{format_err, Error};

use crate::context::{Color, Context};

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
        "directory" => directory::build_segment(&context),
        "user" => user::build_segment(&context),
        "status" => status::build_segment(&context),
        "git_user" => git_user::build_segment(&context),
        _ => Err(format_err!("Unknown segment: {}", name)),
    }
}
