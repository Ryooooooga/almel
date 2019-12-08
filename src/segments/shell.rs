use crate::context::Context;
use crate::segments::Segment;
use crate::shell::Shell;

pub fn build_segment<'ctx>(context: &'ctx Context) -> Option<Segment<'ctx>> {
    let config = &context.config.shell;
    let shell = &context.opt.shell;

    let segment = match shell {
        Shell::Bash => Segment {
            style: &config.bash.style,
            content: config.bash.icon.clone(),
        },
        Shell::Zsh => Segment {
            style: &config.zsh.style,
            content: config.zsh.icon.clone(),
        },
        Shell::Fish => Segment {
            style: &config.fish.style,
            content: config.fish.icon.clone(),
        },
    };

    Some(segment)
}
