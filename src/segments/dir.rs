use crate::prompt::Prompt;

pub fn prompt_segment(_: &Prompt) {
    print!(
        "{} >",
        std::env::current_dir().unwrap().to_str().unwrap_or("?")
    );
}
