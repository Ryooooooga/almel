use crate::Error;

#[derive(Debug, PartialEq)]
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

    pub fn init_script(&self) -> &'static str {
        match self {
            Self::Zsh => include_str!("init.zsh"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_name() {
        assert_eq!(Shell::from_name("zsh").ok(), Some(Shell::Zsh));
        assert_eq!(Shell::from_name("csh").ok(), None);
    }

    #[test]
    fn test_init_script() {
        let zsh = Shell::from_name("zsh").unwrap();

        assert_eq!(zsh.init_script().is_empty(), false);
    }
}
