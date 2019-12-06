use crate::context::Context;
use crate::segments::Segment;

pub fn build_segment(context: &Context) -> Option<Segment> {
    let config = &context.config.git_user;

    let repo = context.git_repo.as_ref()?;
    let git_config = repo.config().ok()?;
    let user = git_config.get_string("user.name").ok()?;

    Some(Segment {
        background: config.background,
        foreground: config.foreground,
        content: format!("{} {}", config.icon, user),
    })
}
