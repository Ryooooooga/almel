use crate::context::Context;
use crate::segments::{Segment, SegmentError};

pub fn build_segment(context: &Context) -> Result<Segment, SegmentError> {
    Ok(Segment {
        background: 0,
        foreground: 0,
        content: format!("{}", context.opt.exit_status),
    })
}
