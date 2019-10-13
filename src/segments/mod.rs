pub mod dir;
pub mod exit_status;
pub mod user;

use crate::prompt::Prompt;

pub fn prompt_segment(p: &mut Prompt, segment_name: &str) {
    match segment_name {
        "user" => user::prompt_segment(p),
        "dir" => dir::prompt_segment(p),
        "exit-status" => exit_status::prompt_segment(p),
        _ => panic!("unknown segment '{}'", segment_name),
    }
}
