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

    let segments = ["user"];

    for segment in &segments {
        match *segment {
            "user" => segments::user::prompt(&p),
            _ => panic!("unknown segment '{}'", segment),
        }
    }

    println!("{} $", shell);
}
