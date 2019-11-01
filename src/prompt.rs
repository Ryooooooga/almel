use failure::Error;

use crate::context::Context;
use crate::opt::PromptArgs;
use crate::segments;

pub fn run(args: &PromptArgs) -> Result<(), Error> {
    let context = Context::new(args)?;

    for name in &context.config.segments {
        match segments::build_segment(&context, name) {
            Ok(segment) => print!("{} ", segment.content),
            Err(error) => eprintln!("{}", error),
        };
    }

    Ok(())
}
