mod init;
mod prompt;
mod segments;

use crate::prompt::{prompt, Opts};
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("almel")
        .version(clap::crate_version!())
        .version_short("v")
        .author("Ryooooooga")
        .about("A ZSH theme inspired by agnoster-zsh-theme")
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("init")
                .about("Initialize the shell prompt")
                .arg(Arg::with_name("shell").help("shell").required(true)),
        )
        .subcommand(
            SubCommand::with_name("prompt")
                .about("Print the prompt")
                .arg(
                    Arg::with_name("exit-status")
                        .help("Exit status of previous command")
                        .long("exit-status")
                        .takes_value(true)
                        .required(true),
                )
                .arg(Arg::with_name("shell").help("Shell name").required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        ("init", Some(args)) => {
            // almel init <shell>
            let shell = args.value_of("shell").unwrap(); // Never be none
            init::init(shell);
        }
        ("prompt", Some(args)) => {
            // almel prompt <shell>
            let shell = args.value_of("shell").unwrap(); // Never be none
            let exit_status = args.value_of("exit-status").unwrap(); // Never be none
            let exit_status = exit_status.parse::<i32>().unwrap();

            prompt(&Opts { shell, exit_status });
        }
        _ => panic!("Unreachable"),
    };
}
