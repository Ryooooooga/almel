use crate::context::Context;
use crate::segments::{Segment, SegmentError};

pub fn build_segment(context: &Context) -> Result<Option<Segment>, SegmentError> {
    let config = &context.config.git_user;

    let repo = match &context.git_repo {
        Some(repo) => repo,
        None => return Ok(None),
    };

    let git_config = repo.config()?;
    let user = git_config.get_string("user.name")?;

    Ok(Some(Segment {
        background: config.background,
        foreground: config.foreground,
        content: format!("{} {}", config.icon, user),
    }))
}
