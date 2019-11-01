use crate::context::Context;
use crate::segments::{Segment, SegmentError};

#[cfg(target_os = "linux")]
pub fn build_segment(context: &Context) -> Result<Option<Segment>, SegmentError> {
    let config = &context.config.os.linux;

    Ok(Some(Segment {
        background: config.background,
        foreground: config.foreground,
        content: config.icon.clone(),
    }))
}

#[cfg(target_os = "macos")]
pub fn build_segment(context: &Context) -> Result<Option<Segment>, SegmentError> {
    let config = &context.config.os.mac;

    Ok(Some(Segment {
        background: config.background,
        foreground: config.foreground,
        content: config.icon.clone(),
    }))
}

#[cfg(target_os = "windows")]
pub fn build_segment(context: &Context) -> Result<Option<Segment>, SegmentError> {
    let config = &context.config.os.windows;

    Ok(Some(Segment {
        background: config.background,
        foreground: config.foreground,
        content: config.icon.clone(),
    }))
}
