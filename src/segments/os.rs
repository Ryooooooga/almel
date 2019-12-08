use crate::context::Context;
use crate::segments::Segment;

pub fn build_segment<'ctx>(context: &'ctx Context) -> Option<Segment<'ctx>> {
    #[cfg(target_os = "linux")]
    let config = &context.config.os.linux;

    #[cfg(target_os = "macos")]
    let config = &context.config.os.mac;

    #[cfg(target_os = "windows")]
    let config = &context.config.os.windows;

    Some(Segment {
        style: &config.style,
        content: config.icon.clone(),
    })
}
