use crate::shell::Shell;
use crate::Error;

pub fn init(shell: Shell) -> Result<(), Error> {
    print!("{}", shell.init_script());

    Ok(())
}
