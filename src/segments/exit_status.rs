use crate::prompt::Prompt;

pub fn prompt_segment(p: &mut Prompt) {
    print!("{} >", p.opts.exit_status);
}
