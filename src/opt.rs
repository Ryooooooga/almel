use structopt::{clap, StructOpt};

use crate::shell::Shell;

#[derive(Debug, StructOpt)]
#[structopt(
    name = clap::crate_name!(),
    version = clap::crate_version!(),
    author = clap::crate_authors!(),
    about = clap::crate_description!(),
    version_short = "v",
    setting(clap::AppSettings::ColoredHelp),
)]
pub struct Opt {
    #[structopt(subcommand)]
    pub subcommand: Subcommand,
}

impl Opt {
    pub fn parse() -> Self {
        Self::from_args()
    }
}

#[derive(Debug, StructOpt)]
pub enum Subcommand {
    #[structopt(about = "Initialize the shell prompt")]
    Init(InitArgs),

    #[structopt(about = "Print the prompt")]
    Prompt(PromptArgs),
}

#[derive(Debug, StructOpt)]
pub struct InitArgs {
    #[structopt(
            help = "Shell name",
            possible_values = &Shell::possible_values(),
            case_insensitive = true,
        )]
    pub shell: Shell,
}

#[derive(Debug, StructOpt)]
pub struct PromptArgs {
    #[structopt(
            help = "Shell name",
            possible_values = &Shell::possible_values(),
            case_insensitive = true,
        )]
    pub shell: Shell,

    #[structopt(help = "Exit status", long = "exit-status", short = "s")]
    pub exit_status: i32,

    #[structopt(help = "Number of jobs running", long = "num-jobs", short = "j")]
    pub num_jobs: i32,

    #[structopt(help = "Command duration", long = "duration", short = "d")]
    pub duration: f64,
}
