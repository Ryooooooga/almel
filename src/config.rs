use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::context::Color;

// OS
#[derive(Debug, Serialize, Deserialize)]
pub struct OsConfig {
    pub linux: OsConfigEntry,
    pub mac: OsConfigEntry,
    pub windows: OsConfigEntry,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsConfigEntry {
    pub background: Color,
    pub foreground: Color,
    pub icon: String,
}

// User
#[derive(Debug, Serialize, Deserialize)]
pub struct UserConfig {
    pub background: Color,
    pub foreground: Color,
    pub display_host: bool,
}

// Status
#[derive(Debug, Serialize, Deserialize)]
pub struct StatusConfig {
    pub icons: StatusConfigIcons,
    pub succeeded: StatusConfigSucceeded,
    pub failed: StatusConfigFailed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusConfigIcons {
    pub succeeded: String,
    pub failed: String,
    pub root: String,
    pub jobs: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusConfigSucceeded {
    pub background: Color,
    pub foreground: Color,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusConfigFailed {
    pub background: Color,
    pub foreground: Color,
    pub display_exit_status: bool,
}

// Directory
#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryConfig {
    pub normal: DirectoryConfigNormal,
    pub error: DirectoryConfigError,
    pub home: String,
    pub shrink: DirectoryConfigShrink,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryConfigNormal {
    pub background: Color,
    pub foreground: Color,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryConfigError {
    pub background: Color,
    pub foreground: Color,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryConfigShrink {
    pub enabled: bool,
    pub max_len: usize,
}

// Git repository
#[derive(Debug, Serialize, Deserialize)]
pub struct GitRepoConfig {
    pub icons: GitRepoConfigIcons,
    pub clean: GitRepoConfigColors,
    pub unstaged: GitRepoConfigColors,
    pub staged: GitRepoConfigColors,
    pub conflicted: GitRepoConfigColors,
    pub display_tag: bool,
    pub commit_hash_len: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitRepoConfigIcons {
    pub branch: String,
    pub tag: String,
    pub commit: String,
    pub modified: String,
    pub added: String,
    pub deleted: String,
    pub added_deleted: String,
    pub conflicted: String,
    pub behind: String,
    pub ahead: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitRepoConfigColors {
    pub background: Color,
    pub foreground: Color,
}

// Git user
#[derive(Debug, Serialize, Deserialize)]
pub struct GitUserConfig {
    pub background: Color,
    pub foreground: Color,
    pub icon: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub os: OsConfig,
    pub user: UserConfig,
    pub directory: DirectoryConfig,
    pub git_repo: GitRepoConfig,
    pub git_user: GitUserConfig,
    pub status: StatusConfig,
    pub segments: Vec<String>,
}

impl Config {
    pub fn load_from_str(s: &str) -> serde_yaml::Result<Config> {
        serde_yaml::from_str(s)
    }
}

lazy_static! {
    pub static ref DEFAULT_CONFIG_STR: &'static str = include_str!("almel.yaml");
}
