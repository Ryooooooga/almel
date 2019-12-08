use ansi_term::Color;
use failure::Error;

use crate::configs::Config;
use crate::context::Context;
use crate::opt::PromptArgs;
use crate::segments;

pub fn run(args: &PromptArgs) -> Result<(), Error> {
    let config = Config::load_from_file_or_create_default(Config::config_path())?;
    let context = Context::new(args, &config)?;
    let shell = &context.opt.shell;
    let separator = &context.config.segment_separators.left_solid;

    for (i, line) in context.config.segments.iter().enumerate() {
        if i > 0 {
            println!();
        }

        let mut prev_bg: Option<Color> = None;

        for name in line {
            match segments::build_segment(&context, name) {
                Ok(Some(segment)) => {
                    let fg = &segment.style.foreground;
                    let bg = segment.style.background;

                    if let Some(prev_bg) = prev_bg {
                        let style = prev_bg.on(bg);

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

                    let style = fg.on(bg);

                    print!(
                        "{}{}{} {} {}{}{}",
                        shell.control_prefix(),
                        style.prefix(),
                        shell.control_suffix(),
                        segment.content,
                        shell.control_prefix(),
                        style.suffix(),
                        shell.control_suffix(),
                    );

                    prev_bg = Some(bg);
                }
                Ok(None) => {}
                Err(error) => {
                    eprintln!("{}", error);
                }
            };
        }

        if let Some(prev_bg) = prev_bg {
            let style = prev_bg.normal();

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
    }

    print!(" ");

    Ok(())
}
