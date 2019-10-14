use crate::shell::Shell;
use crate::Error;

pub fn init(shell: Shell) -> Result<(), Error> {
    match shell {
        Shell::Zsh => print!("{}", include_str!("init.zsh")),
    };

    Ok(())
}
