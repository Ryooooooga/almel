use std::borrow::Cow;

use crate::context::Context;
use crate::segments::{Segment, SegmentError};

#[cfg(target_os = "windows")]
mod users {
    pub fn get_current_username() -> Option<std::ffi::OsString> {
        std::env::var_os("USERNAME")
    }
}

pub fn build_segment(context: &Context) -> Result<Option<Segment>, SegmentError> {
    let config = &context.config.user;

    let username = users::get_current_username();
    let username = username
        .as_ref()
        .map(|u| u.to_string_lossy())
        .unwrap_or_else(|| Cow::from("?"));

    let hostname = hostname::get();
    let hostname = hostname
        .as_ref()
        .map(|h| h.to_string_lossy())
        .unwrap_or_else(|_| Cow::from("?"));

    let content;
    if config.display_host {
        content = format!("{}@{}", username, hostname)
    } else {
        content = format!("{}", username)
    };

    Ok(Some(Segment {
        background: config.background,
        foreground: config.foreground,
        content,
    }))
}
