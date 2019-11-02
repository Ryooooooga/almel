use lazy_static::lazy_static;
use structopt::clap::arg_enum;

use crate::context::Color;

arg_enum! {
    #[derive(Debug)]
    pub enum Shell {
        Zsh,
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
            Self::Zsh => include_str!("init.zsh"),
        }
    }

    pub fn bg_color(&self, color: &Color) -> String {
        match self {
            Self::Zsh => format!("%{{\u{001b}[48;5;{}m%}}", color),
        }
    }

    pub fn fg_color(&self, color: &Color) -> String {
        match self {
            Self::Zsh => format!("%{{\u{001b}[38;5;{}m%}}", color),
        }
    }

    pub fn reset_styles(&self) -> &'static str {
        match self {
            Self::Zsh => "%{\u{001b}[m%}",
        }
    }
}
