use std::borrow::Cow;

use crate::context::Context;
use crate::segments::Segment;

#[cfg(target_os = "windows")]
mod users {
    pub fn get_current_username() -> Option<std::ffi::OsString> {
        std::env::var_os("USERNAME")
    }
}

pub fn build_segment<'ctx>(context: &'ctx Context) -> Option<Segment<'ctx>> {
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

    let content = if config.display_host {
        format!("{}@{}", username, hostname)
    } else {
        format!("{}", username)
    };

    Some(Segment {
        style: &config.style,
        content,
    })
}
