use crate::context::Context;
use crate::opt::PromptArgs;

pub fn run(args: &PromptArgs) {
    let context = Context::new(args);
    print!(
        "%n@%m > {} > {} $ ",
        context.get_current_dir().to_str().unwrap_or("?"),
        context.get_exit_status()
    );
}
