use crate::context::Context;
use crate::segments::Segment;

#[cfg(target_os = "windows")]
fn is_root_user() -> bool {
    false // TODO: for Windows
}

#[cfg(not(target_os = "windows"))]
fn is_root_user() -> bool {
    users::get_current_uid() == 0
}

pub fn build_segment(context: &Context) -> Option<Segment> {
    let config = &context.config.status;

    let style;
    let mut content = String::new();

    if context.opt.exit_status == 0 {
        style = &config.succeeded.style;
        content += &config.icons.succeeded;
    } else {
        style = &config.failed.style;
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

    Some(Segment {
        background: style.background,
        foreground: style.foreground,
        content,
    })
}
