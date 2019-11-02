use crate::context::Context;
use crate::segments::{Segment, SegmentError};

pub fn build_segment(context: &Context) -> Result<Option<Segment>, SegmentError> {
    let config = &context.config.user;
    let shell = &context.opt.shell;

    let content = if config.display_host {
        format!("{}@{}", shell.username(), shell.hostname())
    } else {
        format!("{}", shell.username())
    };

    Ok(Some(Segment {
        background: config.background,
        foreground: config.foreground,
        content,
    }))
}
