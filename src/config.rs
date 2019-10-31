use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub segments: Vec<String>,
}

impl Config {
    pub fn load_from_str(s: &str) -> serde_yaml::Result<Config> {
        serde_yaml::from_str(s)
    }
}

lazy_static! {
    pub static ref DEFAULT_CONFIG: Config =
        Config::load_from_str(include_str!("config.yaml")).expect("Wrong config.yaml!");
}
