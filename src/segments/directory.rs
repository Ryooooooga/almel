use std::path::PathBuf;

use crate::context::Context;
use crate::segments::Segment;

pub fn build_segment(context: &Context) -> Option<Segment> {
    let config = &context.config.directory;

    let mut cwd = context.current_dir.as_path();
    let mut dir = PathBuf::new();

    let style = if cwd.is_dir() {
        &config.normal.style
    } else {
        &config.error.style
    };

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

            let n = if name.starts_with('.') {
                config.shrink.max_len + 1
            } else {
                config.shrink.max_len
            };

            let shorten: String = name.chars().take(n).collect();
            dir.push(shorten);
        }

        if let Some(last) = last {
            dir.push(last)
        }
    } else {
        dir.push(cwd);
    }

    Some(Segment {
        background: style.background,
        foreground: style.foreground,
        content: dir.to_string_lossy().to_string(),
    })
}
