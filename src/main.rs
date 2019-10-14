mod env;
mod glyph;
mod init;
mod prompt;
mod segments;
mod shell;

use clap::{App, Arg, SubCommand};
use failure::Fail;

use crate::prompt::PromptError;
use crate::shell::Shell;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Unsupported shell: {}", 0)]
    UnsupportedShell(String),

    #[fail(display = "Prompt error: {}", 0)]
    PromptError(PromptError),
}

impl From<PromptError> for Error {
    fn from(err: PromptError) -> Error {
        Error::PromptError(err)
    }
}

fn run() -> Result<(), Error> {
    let matches = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .version_short("v")
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("init")
                .about("Initialize the shell prompt")
                .arg(Arg::with_name("shell").help("shell").required(true)),
        )
        .subcommand(
            SubCommand::with_name("prompt")
                .about("Print the prompt")
                .arg(Arg::with_name("shell").help("Shell name").required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        ("init", Some(args)) => {
            // almel init <shell>
            let shell_name = args.value_of("shell").unwrap(); // Will never be none
            let shell = Shell::from_name(shell_name)?;

            init::init(shell)?;
        }
        ("prompt", Some(args)) => {
            // almel prompt <shell>
            let shell_name = args.value_of("shell").unwrap(); // Will never be none
            let shell = Shell::from_name(shell_name)?;

            prompt::prompt(shell)?;
        }
        _ => panic!("Unreachable"),
    };

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
