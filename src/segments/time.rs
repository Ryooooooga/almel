use chrono::{Local, Utc};

use crate::context::Context;
use crate::segments::{Segment, SegmentError};

pub fn build_segment(context: &Context) -> Result<Option<Segment>, SegmentError> {
    let config = &context.config.time;

    let content = if config.utc {
        Utc::now().format(&config.format).to_string()
    } else {
        Local::now().format(&config.format).to_string()
    };

    Ok(Some(Segment {
        background: config.background,
        foreground: config.foreground,
        content,
    }))
}
