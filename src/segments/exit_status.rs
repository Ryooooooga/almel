use crate::prompt::Prompt;

pub fn prompt_segment(p: &mut Prompt) {
    if p.opts.exit_status == 0 {
        // Succeeded
        p.start_segment("black", "white");
        print!("{}", p.opts.exit_status);
    } else {
        // Failed
        p.start_segment("white", "red");
        print!("{}", p.opts.exit_status);
    }
}
