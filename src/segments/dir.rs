use crate::prompt::Prompt;

pub fn prompt_segment(p: &mut Prompt) {
    let current_dir = std::env::current_dir();
    let current_dir = current_dir
        .as_ref()
        .ok()
        .and_then(|s| s.to_str())
        .unwrap_or("?");

    p.opts.shell.set_color("black", "blue");
    print!("{} >", current_dir);
}
