use std::borrow::Cow;
use std::env;
use std::path::PathBuf;

use crate::context::Context;
use crate::segments::{Segment, SegmentError};

pub fn build_segment(context: &Context) -> Result<Option<Segment>, SegmentError> {
    let config = &context.config.venv;

    let venv = match env::var_os("VIRTUAL_ENV") {
        Some(env) => PathBuf::from(env),
        None => return Ok(None),
    };

    let env_name = venv
        .file_name()
        .map(|f| f.to_string_lossy())
        .unwrap_or_else(|| Cow::from("?"));

    Ok(Some(Segment {
        background: config.background,
        foreground: config.foreground,
        content: format!("{} {}", config.icon, env_name),
    }))
}
