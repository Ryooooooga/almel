use crate::segments;

#[derive(Debug)]
pub struct Prompt<'a> {
    shell: &'a str,
}

impl<'a> Prompt<'a> {
    pub fn new(shell: &'a str) -> Prompt {
        Prompt { shell }
    }
}

pub fn prompt(shell: &str) {
    let p = Prompt::new(shell);

    let segments = ["user", "dir"];

    for segment in &segments {
        match *segment {
            "user" => segments::user::prompt_segment(&p),
            "dir" => segments::dir::prompt_segment(&p),
            _ => panic!("unknown segment '{}'", segment),
        }
    }

    println!("{} $", shell);
}
