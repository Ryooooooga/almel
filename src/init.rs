use failure::Error;

use crate::shell::Shell;

pub fn init(shell: Shell) -> Result<(), Error> {
    print!("{}", shell.init_script());

    Ok(())
}
