use crate::Error;

#[derive(Debug)]
pub enum Shell {
    Zsh,
}

impl Shell {
    pub fn from_name(shell_name: &str) -> Result<Shell, Error> {
        match shell_name {
            "zsh" => Ok(Shell::Zsh),
            _ => Err(Error::UnsupportedShell(shell_name.to_string())),
        }
    }
}
