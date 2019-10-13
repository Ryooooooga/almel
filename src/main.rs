mod init;
mod prompt;
mod segments;
mod shell;

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
            let shell_name = args.value_of("shell").unwrap(); // Never be none
            let shell = shell::shell_from_name(shell_name).unwrap(); // TODO: better error handling

            init::init(shell.as_ref());
        }
        ("prompt", Some(args)) => {
            // almel prompt <shell>
            let shell_name = args.value_of("shell").unwrap(); // Never be none
            let shell = shell::shell_from_name(shell_name).unwrap(); // TODO: better error handling

            let exit_status = args.value_of("exit-status").unwrap(); // Never be none
            let exit_status = exit_status.parse::<i32>().unwrap(); // TODO: better error handling

            prompt(&Opts {
                shell: shell.as_ref(),
                exit_status,
            });
        }
        _ => panic!("Unreachable"),
    };
}
