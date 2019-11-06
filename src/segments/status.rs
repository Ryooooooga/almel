use crate::color::Color;
use crate::context::Context;
use crate::segments::{Segment, SegmentError};

#[cfg(target_os = "windows")]
fn is_root_user() -> bool {
    false // TODO: for Windows
}

#[cfg(not(target_os = "windows"))]
fn is_root_user() -> bool {
    users::get_current_uid() == 0
}

pub fn build_segment(context: &Context) -> Result<Option<Segment>, SegmentError> {
    let config = &context.config.status;

    let background: Color;
    let foreground: Color;
    let mut content = String::new();

    if context.opt.exit_status == 0 {
        background = config.succeeded.background;
        foreground = config.succeeded.foreground;
        content += &config.icons.succeeded;
    } else {
        background = config.failed.background;
        foreground = config.failed.foreground;
        content += &config.icons.failed;

        if config.failed.display_exit_status {
            content += &format!(" {}", context.opt.exit_status);
        }
    }

    if is_root_user() {
        content += &format!(" {}", config.icons.root);
    }

    if context.opt.num_jobs > 0 {
        content += &format!(" {}", config.icons.jobs);
    }

    Ok(Some(Segment {
        background,
        foreground,
        content,
    }))
}
