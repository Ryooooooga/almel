use ansi_term::Color;
use failure::Error;

use crate::configs::{Config, SegmentStyle};
use crate::context::Context;
use crate::opt::PromptArgs;
use crate::segments;
use crate::shell::Shell;

fn display_content(shell: &Shell, style: &SegmentStyle, content: &str) {
    // Convert to ansi_term::Style
    let style = style.foreground.on(style.background);

    print!(
        "{}{}{} {} {}{}{}",
        shell.control_prefix(),
        style.prefix(),
        shell.control_suffix(),
        content,
        shell.control_prefix(),
        style.suffix(),
        shell.control_suffix(),
    );
}

fn display_separator(shell: &Shell, style: &SegmentStyle, last_bg: Color, separator: &str) {
    // Convert to ansi_term::Style
    let style = last_bg.on(style.background);

    print!(
        "{}{}{}{}{}{}{}",
        shell.control_prefix(),
        style.prefix(),
        shell.control_suffix(),
        separator,
        shell.control_prefix(),
        style.suffix(),
        shell.control_suffix(),
    );
}

fn display_closure(shell: &Shell, last_bg: Color, separator: &str) {
    // Convert to ansi_term::Style
    let style = last_bg.normal();

    print!(
        "{}{}{}{}{}{}{}",
        shell.control_prefix(),
        style.prefix(),
        shell.control_suffix(),
        separator,
        shell.control_prefix(),
        style.suffix(),
        shell.control_suffix(),
    );
}

pub fn run(args: &PromptArgs) -> Result<(), Error> {
    let config = Config::load_from_file_or_create_default(Config::config_path())?;
    let context = Context::new(args, &config)?;
    let shell = &context.opt.shell;
    let separator = &context.config.segment_separators.left_solid;

    for (i, line) in context.config.segments.iter().enumerate() {
        if i > 0 {
            println!();
        }

        let mut last_bg: Option<Color> = None;

        for name in line {
            match segments::build_segment(&context, name) {
                Ok(Some(segment)) => {
                    if let Some(last_bg) = last_bg {
                        display_separator(shell, &segment.style, last_bg, separator);
                    }

                    display_content(shell, &segment.style, &segment.content);
                    last_bg = Some(segment.style.background);
                }
                Ok(None) => {}
                Err(error) => {
                    eprintln!("{}", error);
                }
            };
        }

        if let Some(last_bg) = last_bg {
            display_closure(shell, last_bg, separator);
        }
    }

    print!(" ");

    Ok(())
}
