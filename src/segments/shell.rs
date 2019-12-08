use crate::context::Context;
use crate::segments::Segment;
use crate::shell::Shell;

pub fn build_segment(context: &Context) -> Option<Segment> {
    let config = &context.config.shell;
    let shell = &context.opt.shell;

    let segment = match shell {
        Shell::Bash => Segment {
            background: config.bash.style.background,
            foreground: config.bash.style.foreground,
            content: config.bash.icon.clone(),
        },
        Shell::Zsh => Segment {
            background: config.zsh.style.background,
            foreground: config.zsh.style.foreground,
            content: config.zsh.icon.clone(),
        },
        Shell::Fish => Segment {
            background: config.fish.style.background,
            foreground: config.fish.style.foreground,
            content: config.fish.icon.clone(),
        },
    };

    Some(segment)
}
