use crate::prompt::Prompt;
pub fn prompt_segment(p: &mut Prompt) {
    p.start_segment("white", "black");
    print!("%n@%m");
}
