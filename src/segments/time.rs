use crate::context::Context;
use crate::segments::{Segment, SegmentError};

pub fn build_segment(context: &Context) -> Result<Option<Segment>, SegmentError> {
    Ok(Some(Segment {
        background: 1,
        foreground: 0,
        content: "%D %*".to_string(),
    }))
}
