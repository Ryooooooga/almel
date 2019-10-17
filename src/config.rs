use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OsEntry {
    background: String,
    foreground: String,
    icon: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Os {
    linux: OsEntry,
    mac: OsEntry,
    windows: OsEntry,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    background: String,
    foreground: String,
    display_host: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Git {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Newline {}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusSucceeded {
    background: String,
    foreground: String,
    icon: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusFailed {
    background: String,
    foreground: String,
    icon: String,
    display_exit_code: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    root_icon: String,
    job_icon: String,
    succeeded: StatusSucceeded,
    failed: StatusFailed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SegmentSeparators {
    left_solid: String,
}

type Segments = Vec<String>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    os: Os,
    user: User,
    git: Git,
    newline: Newline,
    status: Status,
    segment_separators: SegmentSeparators,
    segments: Segments,
}

impl Config {
    pub fn load(path: &str) {
        let config = serde_yaml::from_str::<Config>(path);

        eprintln!("{:#?}", config);
    }
}
