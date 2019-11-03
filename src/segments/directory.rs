use crate::color::Color;
use crate::context::Context;
use crate::segments::{Segment, SegmentError};

pub fn build_segment(context: &Context) -> Result<Option<Segment>, SegmentError> {
    let config = &context.config.directory;
    let cwd = &context.current_dir;

    // TODO: home
    // TODO: shrinking

    let background: Color;
    let foreground: Color;

    if cwd.is_dir() {
        background = config.normal.background;
        foreground = config.normal.foreground;
    } else {
        background = config.error.background;
        foreground = config.error.foreground;
    }

    Ok(Some(Segment {
        background,
        foreground,
        content: cwd.to_string_lossy().to_string(),
    }))
}
