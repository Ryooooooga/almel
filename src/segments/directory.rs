use std::path::PathBuf;

use crate::color::Color;
use crate::context::Context;
use crate::segments::{Segment, SegmentError};

pub fn build_segment(context: &Context) -> Result<Option<Segment>, SegmentError> {
    let config = &context.config.directory;

    let mut cwd = context.current_dir.as_path();
    let mut dir = PathBuf::new();

    let background: Color;
    let foreground: Color;

    if cwd.is_dir() {
        background = config.normal.background;
        foreground = config.normal.foreground;
    } else {
        background = config.error.background;
        foreground = config.error.foreground;
    }

    // Replace home
    if let Some(home) = dirs::home_dir() {
        if let Ok(postfix) = cwd.strip_prefix(&home) {
            cwd = postfix;
            dir.push(&config.home);
        }
    }

    if config.shrink.enabled {
        let mut components = cwd.iter();
        let last = components.next_back();

        for name in components {
            let name = &name.to_string_lossy();

            let n;
            if name.starts_with(".") {
                n = config.shrink.max_len + 1;
            } else {
                n = config.shrink.max_len;
            }

            let shorten: String = name.chars().take(n).collect();
            dir.push(shorten);
        }

        if let Some(last) = last {
            dir.push(last)
        }
    } else {
        dir.push(cwd);
    }

    Ok(Some(Segment {
        background,
        foreground,
        content: dir.to_string_lossy().to_string(),
    }))
}
