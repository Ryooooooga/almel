use std::borrow::Cow;
use std::env;
use std::path::PathBuf;

use crate::context::Context;
use crate::segments::Segment;

pub fn build_segment(context: &Context) -> Option<Segment> {
    let config = &context.config.venv;

    let venv = env::var_os("VIRTUAL_ENV").map(PathBuf::from)?;

    let env_name = venv
        .file_name()
        .map(|f| f.to_string_lossy())
        .unwrap_or_else(|| Cow::from("?"));

    Some(Segment {
        background: config.background,
        foreground: config.foreground,
        content: format!("{} {}", config.icon, env_name),
    })
}
