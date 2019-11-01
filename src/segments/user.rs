use crate::context::Context;
use crate::segments::{Segment, SegmentError};

pub fn build_segment(context: &Context) -> Result<Segment, SegmentError> {
    let content = if context.config.user.display_host {
        "%n@%m".to_string()
    } else {
        "%n".to_string()
    };

    Ok(Segment {
        background: 0,
        foreground: 7,
        content,
    })
}
