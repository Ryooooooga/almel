use crate::context::Context;
use crate::segments::{Segment, SegmentError};

pub fn build_segment(context: &Context) -> Result<Option<Segment>, SegmentError> {
    let config = &context.config.git_user;

    let user = context
        .git_repo
        .as_ref()
        .and_then(|repo| repo.config().ok())
        .and_then(|config| config.get_string("user.name").ok());

    let user = match user {
        Some(user) => user,
        None => return Ok(None),
    };

    Ok(Some(Segment {
        background: config.background,
        foreground: config.foreground,
        content: format!("{} {}", config.icon, user),
    }))
}
