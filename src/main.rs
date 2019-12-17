mod configs;
mod context;
mod init;
mod opt;
mod prompt;
mod segments;
mod shell;

use crate::opt::{Opt, Subcommand};

fn main() {
    let opt = Opt::parse();

    match &opt.subcommand {
        Subcommand::Init(args) => init::run(&args),
        Subcommand::Prompt(args) => prompt::run(&args),
    };
}
