mod config;
mod context;
mod init;
mod opt;
mod prompt;
mod shell;

use opt::{Opt, Subcommand};

fn main() {
    let opt = Opt::parse();

    match &opt.subcommand {
        Subcommand::Init(args) => init::run(&args),
        Subcommand::Prompt(args) => prompt::run(&args),
    }
}
