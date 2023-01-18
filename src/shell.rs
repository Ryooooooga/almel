use structopt::clap::arg_enum;

arg_enum! {
    #[derive(Debug)]
    pub enum Shell {
        Bash,
        Zsh,
        Fish,
    }
}

pub static POSSIBLE_SHELL_VALUES: &[&str] = &["bash", "zsh", "fish"];

impl Shell {
    pub fn init_script(&self, asynchronous: bool) -> &'static str {
        match self {
            Self::Bash => include_str!("init/almel.bash"),
            Self::Zsh if asynchronous => include_str!("init/almel-async.zsh"),
            Self::Zsh => include_str!("init/almel.zsh"),
            Self::Fish => include_str!("init/almel.fish"),
        }
    }

    pub fn escape_content(&self, content: &str) -> String {
        match self {
            Self::Bash => content.replace('\\', "\\\\"),
            Self::Zsh => content.replace('%', "%%"),
            Self::Fish => content.into(),
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
