use crate::context::Context;
use crate::segments::{Segment, SegmentError};

pub fn build_segment(context: &Context) -> Result<Option<Segment>, SegmentError> {
    let config = &context.config.user;

    let content = if config.display_host {
        "%n@%m".to_string()
    } else {
        "%n".to_string()
    };

    Ok(Some(Segment {
        background: config.background,
        foreground: config.foreground,
        content,
    }))
}
