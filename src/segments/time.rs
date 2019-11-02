use chrono::UTC;

use crate::context::Context;
use crate::segments::{Segment, SegmentError};

pub fn build_segment(context: &Context) -> Result<Option<Segment>, SegmentError> {
    let config = &context.config.time;

    Ok(Some(Segment {
        background: config.background,
        foreground: config.foreground,
        content: format!("{} {}", config.icon, UTC::now().format(&config.format)),
    }))
}
