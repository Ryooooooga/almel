use crate::context::Context;
use crate::segments::Segment;
use crate::shell::Shell;

pub fn build_segment(context: &Context) -> Option<Segment> {
    let config = &context.config.shell;
    let shell = &context.opt.shell;

    let segment = match shell {
        Shell::Bash => Segment {
            background: config.bash.background,
            foreground: config.bash.foreground,
            content: config.bash.icon.clone(),
        },
        Shell::Zsh => Segment {
            background: config.zsh.background,
            foreground: config.zsh.foreground,
            content: config.zsh.icon.clone(),
        },
        Shell::Fish => Segment {
            background: config.fish.background,
            foreground: config.fish.foreground,
            content: config.fish.icon.clone(),
        },
    };

    Some(segment)
}
