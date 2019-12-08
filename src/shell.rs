use lazy_static::lazy_static;
use structopt::clap::arg_enum;

arg_enum! {
    #[derive(Debug)]
    pub enum Shell {
        Bash,
        Zsh,
        Fish,
    }
}

impl Shell {
    pub fn possible_values() -> &'static [&'static str] {
        lazy_static! {
            static ref VALUES: Vec<String> =
                Shell::variants().iter().map(|s| s.to_lowercase()).collect();
            static ref REFS: Vec<&'static str> = VALUES.iter().map(|s| s.as_str()).collect();
        }
        &REFS
    }
}

impl Shell {
    pub fn init_script(&self) -> &'static str {
        match self {
            Self::Bash => include_str!("init/almel.bash"),
            Self::Zsh => include_str!("init/almel.zsh"),
            Self::Fish => include_str!("init/almel.fish"),
        }
    }

    pub fn control_prefix(&self) -> &'static str {
        match self {
            Self::Bash => r"\[",
            Self::Zsh => r"%{",
            Self::Fish => r"",
        }
    }

    pub fn control_suffix(&self) -> &'static str {
        match self {
            Self::Bash => r"\]",
            Self::Zsh => r"%}",
            Self::Fish => r"",
        }
    }
}
