use ansi_term::Color;

use crate::configs::{Config, SegmentSeparators, SegmentStyle};
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
        shell.escape_content(content),
        shell.control_prefix(),
        style.suffix(),
        shell.control_suffix(),
    );
}

fn display_separator(
    shell: &Shell,
    next_style: &SegmentStyle,
    prev_bg: Color,
    separators: &SegmentSeparators,
) {
    #![allow(clippy::useless_let_if_seq)]
    let style;
    let separator;

    if prev_bg == next_style.background {
        style = Color::Fixed(8).on(prev_bg);
        separator = &separators.left_wire;
    } else {
        style = prev_bg.on(next_style.background);
        separator = &separators.left_solid;
    }

    print!(
        "{}{}{}{}{}{}{}",
        shell.control_prefix(),
        style.prefix(),
        shell.control_suffix(),
        shell.escape_content(separator),
        shell.control_prefix(),
        style.suffix(),
        shell.control_suffix(),
    );
}

fn display_closure(shell: &Shell, last_bg: Color, separators: &SegmentSeparators) {
    // Convert to ansi_term::Style
    let style = last_bg.normal();

    print!(
        "{}{}{}{}{}{}{}",
        shell.control_prefix(),
        style.prefix(),
        shell.control_suffix(),
        shell.escape_content(&separators.left_solid),
        shell.control_prefix(),
        style.suffix(),
        shell.control_suffix(),
    );
}

pub fn run(args: &PromptArgs) {
    let config = Config::load_from_file_or_create_default(Config::config_path())
        .map_err(|err| {
            eprintln!("{}", err);
        })
        .unwrap_or_default();

    let context = Context::new(args, &config);
    let shell = &context.opt.shell;
    let separators = &context.config.segment_separators;

    for (i, line) in context.config.segments.iter().enumerate() {
        if i > 0 {
            println!();
        }

        let mut prev_bg: Option<Color> = None;

        for name in line {
            match segments::build_segment(&context, name) {
                Ok(Some(segment)) => {
                    if let Some(prev_bg) = prev_bg {
                        display_separator(shell, segment.style, prev_bg, separators);
                    }

                    display_content(shell, segment.style, &segment.content);
                    prev_bg = Some(segment.style.background);
                }
                Ok(None) => {}
                Err(error) => {
                    eprintln!("{}", error);
                }
            };
        }

        if let Some(last_bg) = prev_bg {
            display_closure(shell, last_bg, separators);
        }
    }

    print!(" ");
}
