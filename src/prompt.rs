use failure::Error;

use crate::context::Context;
use crate::opt::PromptArgs;
use crate::segments;

pub fn run(args: &PromptArgs) -> Result<(), Error> {
    let context = Context::new(args)?;

    for (i, line) in context.config.segments.iter().enumerate() {
        if i > 0 {
            println!("");
        }

        for name in line {
            match segments::build_segment(&context, name) {
                Ok(Some(segment)) => print!(
                    "%{{%K{{{}}}%F{{{}}}%}} {} %{{%K{{default}}%F{{default}}%}}",
                    segment.background, segment.foreground, segment.content
                ),
                Ok(None) => {}
                Err(error) => {
                    eprintln!("{}", error);
                }
            };
        }
    }

    print!(" ");

    Ok(())
}
