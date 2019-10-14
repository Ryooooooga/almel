use crate::glyph;
use crate::segments;
use crate::shell::Shell;

pub struct Opts<'a> {
    pub exit_status: i32,
    pub shell: &'a dyn Shell,
}

pub struct Prompt<'a> {
    pub opts: &'a Opts<'a>,
    pub current_bg: Option<String>,
}

impl<'a> Prompt<'a> {
    pub fn new(opts: &'a Opts<'a>) -> Prompt {
        Prompt {
            opts,
            current_bg: None,
        }
    }

    pub fn start_segment<S: Into<String>>(&mut self, foreground: &str, background: S) {
        let background = background.into();

        if let Some(curreng_bg) = &self.current_bg {
            print!(" ");
            self.opts.shell.set_color(&curreng_bg, &background);
            print!("{}", glyph::segment_separator::LEFT_SOLID);
        }

        self.opts.shell.set_color(foreground, &background);
        self.current_bg = Some(background);

        print!(" ");
    }

    pub fn end_segments(&mut self) {
        print!(" ");

        if let Some(curreng_bg) = &self.current_bg {
            self.opts.shell.set_color(&curreng_bg, "default");
            print!("{}", glyph::segment_separator::LEFT_SOLID);
        }

        self.opts.shell.clear_color();
        self.current_bg = None;

        print!(" ");
    }
}

pub fn prompt(opts: &Opts) {
    let mut p = Prompt::new(opts);
    let segments = ["user", "dir", "exit-status"];

    for segment in &segments {
        segments::prompt_segment(&mut p, segment);
    }

    p.end_segments();
}
