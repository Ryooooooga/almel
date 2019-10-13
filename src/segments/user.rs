use crate::prompt::Prompt;
pub fn prompt_segment(p: &mut Prompt) {
    p.opts.shell.set_color("white", "black");
    print!(" %n@%m >");
}
