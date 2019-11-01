use failure::Error;

use crate::opt::InitArgs;

pub fn run(args: &InitArgs) -> Result<(), Error> {
    print!("{}", args.shell.init_script());

    Ok(())
}
