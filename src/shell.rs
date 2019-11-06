use lazy_static::lazy_static;
use structopt::clap::arg_enum;

use crate::color::Color;

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

    pub fn bg_color(&self, color: Color) -> String {
        match self {
            Self::Bash => format!("\\[\u{001b}[48;5;{}m\\]", color),
            Self::Zsh => format!("%{{\u{001b}[48;5;{}m%}}", color),
            Self::Fish => format!("\u{001b}[48;5;{}m", color),
        }
    }

    pub fn fg_color(&self, color: Color) -> String {
        match self {
            Self::Bash => format!("\\[\u{001b}[38;5;{}m\\]", color),
            Self::Zsh => format!("%{{\u{001b}[38;5;{}m%}}", color),
            Self::Fish => format!("\u{001b}[38;5;{}m", color),
        }
    }

    pub fn reset_styles(&self) -> &'static str {
        match self {
            Self::Bash => "\\[\u{001b}[m\\]",
            Self::Zsh => "%{\u{001b}[m%}",
            Self::Fish => "\u{001b}[m",
        }
    }
}
