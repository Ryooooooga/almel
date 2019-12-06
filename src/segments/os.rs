use crate::context::Context;
use crate::segments::Segment;

pub fn build_segment(context: &Context) -> Option<Segment> {
    #[cfg(target_os = "linux")]
    let config = &context.config.os.linux;

    #[cfg(target_os = "macos")]
    let config = &context.config.os.mac;

    #[cfg(target_os = "windows")]
    let config = &context.config.os.windows;

    Some(Segment {
        background: config.background,
        foreground: config.foreground,
        content: config.icon.clone(),
    })
}
