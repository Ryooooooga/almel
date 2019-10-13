use crate::prompt::Prompt;

pub fn prompt_segment(p: &mut Prompt) {
    if p.opts.exit_status == 0 {
        // Succeeded
        p.opts.shell.set_color("black", "white");
        print!(" {} >", p.opts.exit_status);
    } else {
        // Failed
        p.opts.shell.set_color("white", "red");
        print!(" {} >", p.opts.exit_status);
    }
}
