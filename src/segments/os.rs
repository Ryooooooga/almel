use crate::context::Context;
use crate::segments::{Segment, SegmentError};

#[cfg(target_os = "linux")]
pub fn build_segment(context: &Context) -> Result<Segment, SegmentError> {
    Ok(Segment {
        background: context.config.os.linux.background,
        foreground: context.config.os.linux.foreground,
        content: context.config.os.linux.icon.clone(),
    })
}

#[cfg(target_os = "macos")]
pub fn build_segment(context: &Context) -> Result<Segment, SegmentError> {
    Ok(Segment {
        background: context.config.os.mac.background,
        foreground: context.config.os.mac.foreground,
        content: context.config.os.mac.icon.clone(),
    })
}

#[cfg(target_os = "windows")]
pub fn build_segment(context: &Context) -> Result<Segment, SegmentError> {
    Ok(Segment {
        background: context.config.os.windows.background,
        foreground: context.config.os.windows.foreground,
        content: context.config.os.windows.icon.clone(),
    })
}
