use chrono::{Local, Utc};

use crate::context::Context;
use crate::segments::Segment;

pub fn build_segment<'ctx>(context: &'ctx Context) -> Option<Segment<'ctx>> {
    let config = &context.config.time;

    let content = if config.utc {
        Utc::now().format(&config.format).to_string()
    } else {
        Local::now().format(&config.format).to_string()
    };

    Some(Segment {
        style: &config.style,
        content,
    })
}
