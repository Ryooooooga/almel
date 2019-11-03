use chrono::{Local, UTC};

use crate::context::Context;
use crate::segments::{Segment, SegmentError};

pub fn build_segment(context: &Context) -> Result<Option<Segment>, SegmentError> {
    let config = &context.config.time;

    let time;

    if config.utc {
        time = UTC::now().format(&config.format);
    } else {
        time = Local::now().format(&config.format);
    }

    Ok(Some(Segment {
        background: config.background,
        foreground: config.foreground,
        content: format!("{} {}", config.icon, time),
    }))
}
