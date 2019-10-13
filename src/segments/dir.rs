use crate::prompt::Prompt;

pub fn prompt_segment(_: &mut Prompt) {
    print!(
        "{} >",
        std::env::current_dir().unwrap().to_str().unwrap_or("?")
    );
}
