use crate::opt::InitArgs;

pub fn run(args: &InitArgs) {
    print!("{}", args.shell.init_script(args.asynchronous));
}
