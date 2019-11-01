use crate::context::Context;
use crate::segments::{Segment, SegmentError};

pub fn build_segment(context: &Context) -> Result<Segment, SegmentError> {
    Ok(Segment {
        foreground: 0,
        background: 1,
        content: "%n@%m".to_string(),
    })
}
