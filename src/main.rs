mod init;
mod opt;
mod shell;

use opt::{Opt, Subcommand};

fn main() {
    let opt = Opt::parse();

    match &opt.subcommand {
        Subcommand::Init(args) => init::run(&args),
        Subcommand::Prompt(_args) => unimplemented!(),
    }
}
