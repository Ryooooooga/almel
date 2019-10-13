use crate::prompt::Prompt;

pub fn prompt_segment(p: &Prompt) {
    print!("{} >", p.opts.exit_status);
}
