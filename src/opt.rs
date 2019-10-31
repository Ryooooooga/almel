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
    Init {
        #[structopt(
            help = "Shell name",
            possible_values = &Shell::possible_values(),
            case_insensitive = true,
        )]
        shell: Shell,
    },

    #[structopt(about = "Print the prompt")]
    Prompt {
        #[structopt(
            help = "Shell name",
            possible_values = &Shell::possible_values(),
            case_insensitive = true,
        )]
        shell: Shell,

        #[structopt(help = "Exit status", long = "exit-status", short = "s")]
        exit_status: i32,

        #[structopt(help = "Number of jobs running", long = "num-jobs", short = "j")]
        num_jobs: i32,
    },
}
