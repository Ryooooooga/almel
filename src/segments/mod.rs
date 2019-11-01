mod directory;
mod host;
mod os;
mod status;

use failure::{format_err, Error};

use crate::context::Context;

type SegmentError = Error;

#[derive(Debug)]
pub struct Segment {
    pub foreground: i32,
    pub background: i32,
    pub content: String,
}

pub fn build_segment(context: &Context, name: &str) -> Result<Segment, SegmentError> {
    match name {
        "os" => os::build_segment(&context),
        "directory" => directory::build_segment(&context),
        "host" => host::build_segment(&context),
        "status" => status::build_segment(&context),
        _ => Err(format_err!("Unknown segment: {}", name)),
    }
}
