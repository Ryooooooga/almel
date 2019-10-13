use crate::shell::Shell;

pub fn init(shell: &dyn Shell) {
    shell.print_init();
}
