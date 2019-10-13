use crate::segments;
use crate::shell::Shell;

pub struct Opts<'a> {
    pub exit_status: i32,
    pub shell: &'a dyn Shell,
}

pub struct Prompt<'a> {
    pub opts: &'a Opts<'a>,
}

impl<'a> Prompt<'a> {
    pub fn new(opts: &'a Opts<'a>) -> Prompt {
        Prompt { opts }
    }
}

pub fn prompt(opts: &Opts) {
    let mut p = Prompt::new(opts);
    let segments = ["user", "dir", "exit-status"];

    for segment in &segments {
        segments::prompt_segment(&mut p, segment);
    }

    p.opts.shell.clear_color();
}
