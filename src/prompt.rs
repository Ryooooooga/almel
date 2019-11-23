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

        let mut prev_bg = None;

        for name in line {
            match segments::build_segment(&context, name) {
                Ok(Some(segment)) => {
                    if let Some(prev_bg) = &prev_bg {
                        print!(
                            "{}{}{}",
                            shell.bg_color(segment.background),
                            shell.fg_color(*prev_bg),
                            separator
                        );
                    }

                    print!(
                        "{}{} {} ",
                        shell.bg_color(segment.background),
                        shell.fg_color(segment.foreground),
                        segment.content,
                    );

                    prev_bg = Some(segment.background);
                }
                Ok(None) => {}
                Err(error) => {
                    eprintln!("{}", error);
                }
            };
        }

        if let Some(prev_bg) = prev_bg {
            print!(
                "{}{}{}",
                shell.reset_styles(),
                shell.fg_color(prev_bg),
                separator
            );
        }
    }

    print!("{} ", shell.reset_styles());

    Ok(())
}
