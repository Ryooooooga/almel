use chrono::{Local, UTC};

use crate::context::Context;
use crate::segments::{Segment, SegmentError};

pub fn build_segment(context: &Context) -> Result<Option<Segment>, SegmentError> {
    let config = &context.config.time;

    let content;
    if config.utc {
        content = UTC::now().format(&config.format).to_string();
    } else {
        content = Local::now().format(&config.format).to_string();
    }

    Ok(Some(Segment {
        background: config.background,
        foreground: config.foreground,
        content,
    }))
}
