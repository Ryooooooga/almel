use crate::context::Context;
use crate::segments::{Segment, SegmentError};

pub fn build_segment(context: &Context) -> Result<Segment, SegmentError> {
    let cwd = &context.current_dir;

    let background: i32;
    let foreground: i32;

    if cwd.is_dir() {
        background = 1;
        foreground = 0;
    } else {
        background = 3;
        foreground = 0;
    }

    Ok(Segment {
        background,
        foreground,
        content: cwd.to_string_lossy().to_string(),
    })
}
