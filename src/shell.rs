#[derive(Debug)]
pub enum Error {
    UnknownShell(String),
}

pub trait Shell {
    fn name(&self) -> &'static str;

    fn print_init(&self);

    fn set_color(&self, foreground: &str, background: &str);
    fn clear_color(&self);
}

pub fn shell_from_name(shell_name: &str) -> Result<Box<dyn Shell>, Error> {
    match shell_name {
        "zsh" => Ok(Box::new(Zsh {})),
        _ => Err(Error::UnknownShell(String::from(shell_name))),
    }
}

#[derive(Debug)]
pub struct Zsh {}

impl Shell for Zsh {
    fn name(&self) -> &'static str {
        "zsh"
    }

    fn print_init(&self) {
        print!("{}", include_str!("init.zsh"));
    }

    fn set_color(&self, foreground: &str, background: &str) {
        print!("%{{%F{{{}}}%K{{{}}}%}}", foreground, background);
    }

    fn clear_color(&self) {
        self.set_color("default", "default");
    }
}
