use crate::context::Context;
use crate::segments::{Segment, SegmentError};
use crate::shell::Shell;

pub fn build_segment(context: &Context) -> Result<Option<Segment>, SegmentError> {
    let config = &context.config.shell;
    let shell = &context.opt.shell;

    let config = match shell {
        Shell::Bash => &config.bash,
        Shell::Zsh => &config.zsh,
        Shell::Fish => &config.fish,
    };

    Ok(Some(Segment {
        background: config.background,
        foreground: config.foreground,
        content: config.icon.clone(),
    }))
}
