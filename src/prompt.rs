use crate::segments;

#[derive(Debug)]
pub struct Opts<'a> {
    pub exit_status: i32,
    pub shell: &'a str,
}

#[derive(Debug)]
pub struct Prompt<'a> {
    pub opts: &'a Opts<'a>,
}

impl<'a> Prompt<'a> {
    pub fn new(opts: &'a Opts<'a>) -> Prompt {
        Prompt { opts }
    }
}

pub fn prompt(opts: &Opts) {
    let p = Prompt::new(opts);

    let segments = ["user", "dir", "exit-status"];

    for segment in &segments {
        match *segment {
            "user" => segments::user::prompt_segment(&p),
            "dir" => segments::dir::prompt_segment(&p),
            "exit-status" => segments::exit_status::prompt_segment(&p),
            _ => panic!("unknown segment '{}'", segment),
        }
    }

    println!("{} $", opts.shell);
}
