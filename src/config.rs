use failure::Error;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::fs::{create_dir_all, File};
use std::io::prelude::Write; // File#write_all
use std::path::{Path, PathBuf};

use crate::color;
use crate::color::Color;

// OS
#[derive(Debug, Serialize, Deserialize)]
pub struct OsConfig {
    #[serde(default)]
    pub linux: OsConfigLinux,

    #[serde(default)]
    pub mac: OsConfigMac,

    #[serde(default)]
    pub windows: OsConfigWindows,
}
impl Default for OsConfig {
    fn default() -> Self {
        Self {
            linux: OsConfigLinux::default(),
            mac: OsConfigMac::default(),
            windows: OsConfigWindows::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsConfigLinux {
    #[serde(default = "OsConfigLinux::default_background")]
    pub background: Color,

    #[serde(default = "OsConfigLinux::default_foreground")]
    pub foreground: Color,

    #[serde(default = "OsConfigLinux::default_icon")]
    pub icon: String,
}
impl OsConfigLinux {
    fn default_background() -> Color {
        color::CYAN
    }
    fn default_foreground() -> Color {
        color::WHITE
    }
    fn default_icon() -> String {
        "\u{f17c}".to_string() // nf-fa-linux
    }
}
impl Default for OsConfigLinux {
    fn default() -> Self {
        Self {
            background: Self::default_background(),
            foreground: Self::default_foreground(),
            icon: Self::default_icon(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsConfigMac {
    #[serde(default = "OsConfigMac::default_background")]
    pub background: Color,

    #[serde(default = "OsConfigMac::default_foreground")]
    pub foreground: Color,

    #[serde(default = "OsConfigMac::default_icon")]
    pub icon: String,
}
impl OsConfigMac {
    fn default_background() -> Color {
        color::CYAN
    }
    fn default_foreground() -> Color {
        color::WHITE
    }
    fn default_icon() -> String {
        "\u{f179}".to_string() // nf-fa-apple
    }
}
impl Default for OsConfigMac {
    fn default() -> Self {
        Self {
            background: Self::default_background(),
            foreground: Self::default_foreground(),
            icon: Self::default_icon(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OsConfigWindows {
    #[serde(default = "OsConfigWindows::default_background")]
    pub background: Color,

    #[serde(default = "OsConfigWindows::default_foreground")]
    pub foreground: Color,

    #[serde(default = "OsConfigWindows::default_icon")]
    pub icon: String,
}
impl OsConfigWindows {
    fn default_background() -> Color {
        color::CYAN
    }
    fn default_foreground() -> Color {
        color::WHITE
    }
    fn default_icon() -> String {
        "\u{f17a}".to_string() // nf-fa-windows
    }
}
impl Default for OsConfigWindows {
    fn default() -> Self {
        Self {
            background: Self::default_background(),
            foreground: Self::default_foreground(),
            icon: Self::default_icon(),
        }
    }
}

// Shell
#[derive(Debug, Serialize, Deserialize)]
pub struct ShellConfig {
    #[serde(default)]
    pub bash: ShellConfigBash,

    #[serde(default)]
    pub zsh: ShellConfigZsh,

    #[serde(default)]
    pub fish: ShellConfigFish,
}
impl Default for ShellConfig {
    fn default() -> Self {
        Self {
            bash: ShellConfigBash::default(),
            zsh: ShellConfigZsh::default(),
            fish: ShellConfigFish::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShellConfigBash {
    #[serde(default = "ShellConfigBash::default_background")]
    pub background: Color,

    #[serde(default = "ShellConfigBash::default_foreground")]
    pub foreground: Color,

    #[serde(default = "ShellConfigBash::default_icon")]
    pub icon: String,
}
impl ShellConfigBash {
    fn default_background() -> Color {
        color::WHITE
    }
    fn default_foreground() -> Color {
        color::GREEN
    }
    fn default_icon() -> String {
        "Bash".to_string()
    }
}
impl Default for ShellConfigBash {
    fn default() -> Self {
        Self {
            background: Self::default_background(),
            foreground: Self::default_foreground(),
            icon: Self::default_icon(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShellConfigZsh {
    #[serde(default = "ShellConfigZsh::default_background")]
    pub background: Color,

    #[serde(default = "ShellConfigZsh::default_foreground")]
    pub foreground: Color,

    #[serde(default = "ShellConfigZsh::default_icon")]
    pub icon: String,
}
impl ShellConfigZsh {
    fn default_background() -> Color {
        color::WHITE
    }
    fn default_foreground() -> Color {
        color::GREEN
    }
    fn default_icon() -> String {
        "Zsh".to_string()
    }
}
impl Default for ShellConfigZsh {
    fn default() -> Self {
        Self {
            background: Self::default_background(),
            foreground: Self::default_foreground(),
            icon: Self::default_icon(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShellConfigFish {
    #[serde(default = "ShellConfigFish::default_background")]
    pub background: Color,

    #[serde(default = "ShellConfigFish::default_foreground")]
    pub foreground: Color,

    #[serde(default = "ShellConfigFish::default_icon")]
    pub icon: String,
}
impl ShellConfigFish {
    fn default_background() -> Color {
        color::WHITE
    }
    fn default_foreground() -> Color {
        color::GREEN
    }
    fn default_icon() -> String {
        "\u{f739}".to_string() // nf-mdi-fish
    }
}
impl Default for ShellConfigFish {
    fn default() -> Self {
        Self {
            background: Self::default_background(),
            foreground: Self::default_foreground(),
            icon: Self::default_icon(),
        }
    }
}

// User
#[derive(Debug, Serialize, Deserialize)]
pub struct UserConfig {
    #[serde(default = "UserConfig::default_background")]
    pub background: Color,

    #[serde(default = "UserConfig::default_foreground")]
    pub foreground: Color,

    #[serde(default = "UserConfig::default_display_host")]
    pub display_host: bool,
}

impl UserConfig {
    fn default_background() -> Color {
        color::BLACK
    }
    fn default_foreground() -> Color {
        color::WHITE
    }
    fn default_display_host() -> bool {
        true
    }
}
impl Default for UserConfig {
    fn default() -> Self {
        Self {
            background: Self::default_background(),
            foreground: Self::default_foreground(),
            display_host: Self::default_display_host(),
        }
    }
}

// Status
#[derive(Debug, Serialize, Deserialize)]
pub struct StatusConfig {
    #[serde(default)]
    pub icons: StatusConfigIcons,

    #[serde(default)]
    pub succeeded: StatusConfigSucceeded,

    #[serde(default)]
    pub failed: StatusConfigFailed,
}
impl Default for StatusConfig {
    fn default() -> Self {
        Self {
            icons: StatusConfigIcons::default(),
            succeeded: StatusConfigSucceeded::default(),
            failed: StatusConfigFailed::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusConfigIcons {
    #[serde(default = "StatusConfigIcons::default_succeeded")]
    pub succeeded: String,

    #[serde(default = "StatusConfigIcons::default_failed")]
    pub failed: String,

    #[serde(default = "StatusConfigIcons::default_root")]
    pub root: String,

    #[serde(default = "StatusConfigIcons::default_jobs")]
    pub jobs: String,
}
impl StatusConfigIcons {
    fn default_succeeded() -> String {
        "✓".to_string()
    }
    fn default_failed() -> String {
        "\u{f06a}".to_string() // nf-fa-exclamation_circle
    }
    fn default_root() -> String {
        "\u{e00a}".to_string() // nf-pom-external_interruption
    }
    fn default_jobs() -> String {
        "\u{f013}".to_string() // nf-fa-gear
    }
}
impl Default for StatusConfigIcons {
    fn default() -> Self {
        Self {
            succeeded: Self::default_succeeded(),
            failed: Self::default_failed(),
            root: Self::default_root(),
            jobs: Self::default_jobs(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusConfigSucceeded {
    #[serde(default = "StatusConfigSucceeded::default_background")]
    pub background: Color,

    #[serde(default = "StatusConfigSucceeded::default_foreground")]
    pub foreground: Color,
}
impl StatusConfigSucceeded {
    fn default_background() -> Color {
        color::WHITE
    }
    fn default_foreground() -> Color {
        color::BLUE
    }
}
impl Default for StatusConfigSucceeded {
    fn default() -> Self {
        Self {
            background: Self::default_background(),
            foreground: Self::default_foreground(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusConfigFailed {
    #[serde(default = "StatusConfigFailed::default_background")]
    pub background: Color,

    #[serde(default = "StatusConfigFailed::default_foreground")]
    pub foreground: Color,

    #[serde(default = "StatusConfigFailed::default_display_exit_status")]
    pub display_exit_status: bool,
}
impl StatusConfigFailed {
    fn default_background() -> Color {
        color::RED
    }
    fn default_foreground() -> Color {
        color::WHITE
    }
    fn default_display_exit_status() -> bool {
        true
    }
}
impl Default for StatusConfigFailed {
    fn default() -> Self {
        Self {
            background: Self::default_background(),
            foreground: Self::default_foreground(),
            display_exit_status: Self::default_display_exit_status(),
        }
    }
}

// Directory
#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryConfig {
    #[serde(default)]
    pub normal: DirectoryConfigNormal,

    #[serde(default)]
    pub error: DirectoryConfigError,

    #[serde(default = "DirectoryConfig::default_home")]
    pub home: String,

    #[serde(default)]
    pub shrink: DirectoryConfigShrink,
}
impl DirectoryConfig {
    fn default_home() -> String {
        "~".to_string()
    }
}
impl Default for DirectoryConfig {
    fn default() -> Self {
        Self {
            normal: DirectoryConfigNormal::default(),
            error: DirectoryConfigError::default(),
            home: Self::default_home(),
            shrink: DirectoryConfigShrink::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryConfigNormal {
    #[serde(default = "DirectoryConfigNormal::default_background")]
    pub background: Color,

    #[serde(default = "DirectoryConfigNormal::default_foreground")]
    pub foreground: Color,
}
impl DirectoryConfigNormal {
    fn default_background() -> Color {
        color::BLUE
    }
    fn default_foreground() -> Color {
        color::BLACK
    }
}
impl Default for DirectoryConfigNormal {
    fn default() -> Self {
        Self {
            background: Self::default_background(),
            foreground: Self::default_foreground(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryConfigError {
    #[serde(default = "DirectoryConfigError::default_background")]
    pub background: Color,

    #[serde(default = "DirectoryConfigError::default_foreground")]
    pub foreground: Color,
}
impl DirectoryConfigError {
    fn default_background() -> Color {
        color::RED
    }
    fn default_foreground() -> Color {
        color::BLACK
    }
}
impl Default for DirectoryConfigError {
    fn default() -> Self {
        Self {
            background: Self::default_background(),
            foreground: Self::default_foreground(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryConfigShrink {
    #[serde(default = "DirectoryConfigShrink::default_enabled")]
    pub enabled: bool,

    #[serde(default = "DirectoryConfigShrink::default_max_len")]
    pub max_len: usize,
}
impl DirectoryConfigShrink {
    fn default_enabled() -> bool {
        true
    }
    fn default_max_len() -> usize {
        1
    }
}
impl Default for DirectoryConfigShrink {
    fn default() -> Self {
        Self {
            enabled: Self::default_enabled(),
            max_len: Self::default_max_len(),
        }
    }
}

// Time
#[derive(Debug, Serialize, Deserialize)]
pub struct TimeConfig {
    #[serde(default = "TimeConfig::default_background")]
    pub background: Color,

    #[serde(default = "TimeConfig::default_foreground")]
    pub foreground: Color,

    #[serde(default = "TimeConfig::default_format")]
    pub format: String,

    #[serde(default = "TimeConfig::default_utc")]
    pub utc: bool,
}
impl TimeConfig {
    fn default_background() -> Color {
        color::GRAY
    }
    fn default_foreground() -> Color {
        color::WHITE
    }
    fn default_format() -> String {
        // nf-fa-clock_o
        "\u{f017} %Y/%m/%d %H:%M:%S%.3f".to_string()
    }
    fn default_utc() -> bool {
        false
    }
}
impl Default for TimeConfig {
    fn default() -> Self {
        Self {
            background: Self::default_background(),
            foreground: Self::default_foreground(),
            format: Self::default_format(),
            utc: Self::default_utc(),
        }
    }
}

// Git repository
#[derive(Debug, Serialize, Deserialize)]
pub struct GitRepoConfig {
    #[serde(default)]
    pub icons: GitRepoConfigIcons,

    #[serde(default)]
    pub clean: GitRepoConfigClean,

    #[serde(default)]
    pub unstaged: GitRepoConfigUnstaged,

    #[serde(default)]
    pub staged: GitRepoConfigStaged,

    #[serde(default)]
    pub conflicted: GitRepoConfigConflicted,

    #[serde(default = "GitRepoConfig::default_display_master")]
    pub display_master: bool,

    #[serde(default = "GitRepoConfig::default_display_tag")]
    pub display_tag: bool,

    #[serde(default = "GitRepoConfig::default_commit_hash_len")]
    pub commit_hash_len: usize,
}
impl GitRepoConfig {
    fn default_display_master() -> bool {
        true
    }
    fn default_display_tag() -> bool {
        true
    }
    fn default_commit_hash_len() -> usize {
        6
    }
}
impl Default for GitRepoConfig {
    fn default() -> Self {
        Self {
            icons: GitRepoConfigIcons::default(),
            clean: GitRepoConfigClean::default(),
            unstaged: GitRepoConfigUnstaged::default(),
            staged: GitRepoConfigStaged::default(),
            conflicted: GitRepoConfigConflicted::default(),
            display_master: Self::default_display_master(),
            display_tag: Self::default_display_tag(),
            commit_hash_len: Self::default_commit_hash_len(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitRepoConfigIcons {
    #[serde(default = "GitRepoConfigIcons::default_branch")]
    pub branch: String,

    #[serde(default = "GitRepoConfigIcons::default_tag")]
    pub tag: String,

    #[serde(default = "GitRepoConfigIcons::default_commit")]
    pub commit: String,

    #[serde(default = "GitRepoConfigIcons::default_modified")]
    pub modified: String,

    #[serde(default = "GitRepoConfigIcons::default_added")]
    pub added: String,

    #[serde(default = "GitRepoConfigIcons::default_deleted")]
    pub deleted: String,

    #[serde(default = "GitRepoConfigIcons::default_added_deleted")]
    pub added_deleted: String,

    #[serde(default = "GitRepoConfigIcons::default_conflicted")]
    pub conflicted: String,

    #[serde(default = "GitRepoConfigIcons::default_behind")]
    pub behind: String,

    #[serde(default = "GitRepoConfigIcons::default_ahead")]
    pub ahead: String,
}
impl GitRepoConfigIcons {
    fn default_branch() -> String {
        "\u{f418}".to_string() // nf-oct-git_branch
    }
    fn default_tag() -> String {
        "\u{f412}".to_string() // nf-oct-tag
    }
    fn default_commit() -> String {
        "\u{f417}".to_string() // nf-oct-git_commit
    }
    fn default_modified() -> String {
        "…".to_string()
    }
    fn default_added() -> String {
        "+".to_string()
    }
    fn default_deleted() -> String {
        "-".to_string()
    }
    fn default_added_deleted() -> String {
        "±".to_string()
    }
    fn default_conflicted() -> String {
        "\u{f47f}".to_string() // nf-oct-git_compare
    }
    fn default_behind() -> String {
        "\u{f175}".to_string() // nf-fa-long_arrow_down
    }
    fn default_ahead() -> String {
        "\u{f176}".to_string() // nf-fa-long_arrow_up
    }
}
impl Default for GitRepoConfigIcons {
    fn default() -> Self {
        Self {
            branch: Self::default_branch(),
            tag: Self::default_tag(),
            commit: Self::default_commit(),
            modified: Self::default_modified(),
            added: Self::default_added(),
            deleted: Self::default_deleted(),
            added_deleted: Self::default_added_deleted(),
            conflicted: Self::default_conflicted(),
            behind: Self::default_behind(),
            ahead: Self::default_ahead(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitRepoConfigClean {
    #[serde(default = "GitRepoConfigClean::default_background")]
    pub background: Color,

    #[serde(default = "GitRepoConfigClean::default_foreground")]
    pub foreground: Color,
}
impl GitRepoConfigClean {
    fn default_background() -> Color {
        color::GREEN
    }
    fn default_foreground() -> Color {
        color::BLACK
    }
}
impl Default for GitRepoConfigClean {
    fn default() -> Self {
        Self {
            background: Self::default_background(),
            foreground: Self::default_foreground(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitRepoConfigUnstaged {
    #[serde(default = "GitRepoConfigUnstaged::default_background")]
    pub background: Color,

    #[serde(default = "GitRepoConfigUnstaged::default_foreground")]
    pub foreground: Color,
}
impl GitRepoConfigUnstaged {
    fn default_background() -> Color {
        color::YELLOW
    }
    fn default_foreground() -> Color {
        color::BLACK
    }
}
impl Default for GitRepoConfigUnstaged {
    fn default() -> Self {
        Self {
            background: Self::default_background(),
            foreground: Self::default_foreground(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitRepoConfigStaged {
    #[serde(default = "GitRepoConfigStaged::default_background")]
    pub background: Color,

    #[serde(default = "GitRepoConfigStaged::default_foreground")]
    pub foreground: Color,
}
impl GitRepoConfigStaged {
    fn default_background() -> Color {
        color::GREEN
    }
    fn default_foreground() -> Color {
        color::BLACK
    }
}
impl Default for GitRepoConfigStaged {
    fn default() -> Self {
        Self {
            background: Self::default_background(),
            foreground: Self::default_foreground(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitRepoConfigConflicted {
    #[serde(default = "GitRepoConfigConflicted::default_background")]
    pub background: Color,

    #[serde(default = "GitRepoConfigConflicted::default_foreground")]
    pub foreground: Color,
}
impl GitRepoConfigConflicted {
    fn default_background() -> Color {
        color::RED
    }
    fn default_foreground() -> Color {
        color::BLACK
    }
}
impl Default for GitRepoConfigConflicted {
    fn default() -> Self {
        Self {
            background: Self::default_background(),
            foreground: Self::default_foreground(),
        }
    }
}

// Git user
#[derive(Debug, Serialize, Deserialize)]
pub struct GitUserConfig {
    #[serde(default = "GitUserConfig::default_background")]
    pub background: Color,

    #[serde(default = "GitUserConfig::default_foreground")]
    pub foreground: Color,

    #[serde(default = "GitUserConfig::default_icon")]
    pub icon: String,
}
impl GitUserConfig {
    fn default_background() -> Color {
        color::CYAN
    }
    fn default_foreground() -> Color {
        color::BLACK
    }
    fn default_icon() -> String {
        "\u{f2c0}".to_string() // nf-fa-user_o
    }
}
impl Default for GitUserConfig {
    fn default() -> Self {
        Self {
            background: Self::default_background(),
            foreground: Self::default_foreground(),
            icon: Self::default_icon(),
        }
    }
}

// Separators
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigSegmentSeparators {
    #[serde(default = "ConfigSegmentSeparators::default_left_solid")]
    pub left_solid: String,
}
impl ConfigSegmentSeparators {
    fn default_left_solid() -> String {
        "\u{e0b0}".to_string() // nf-pl-left_hard_divider
    }
}
impl Default for ConfigSegmentSeparators {
    fn default() -> Self {
        Self {
            left_solid: Self::default_left_solid(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub os: OsConfig,

    #[serde(default)]
    pub shell: ShellConfig,

    #[serde(default)]
    pub user: UserConfig,

    #[serde(default)]
    pub directory: DirectoryConfig,

    #[serde(default)]
    pub git_repo: GitRepoConfig,

    #[serde(default)]
    pub git_user: GitUserConfig,

    #[serde(default)]
    pub status: StatusConfig,

    #[serde(default)]
    pub time: TimeConfig,

    #[serde(default)]
    pub segment_separators: ConfigSegmentSeparators,

    #[serde(default = "Config::default_segments")]
    pub segments: Vec<Vec<String>>,
}
impl Config {
    fn default_segments() -> Vec<Vec<String>> {
        [
            vec!["os", "shell", "user", "directory", "git_repo", "git_user"],
            vec!["time", "status"],
        ]
        .iter()
        .map(|line| line.iter().map(|s| s.to_string()).collect())
        .collect()
    }
}

type ConfigError = Error;

impl Config {
    pub fn load_from_str(s: &str) -> Result<Self, ConfigError> {
        let config = serde_yaml::from_str(s)?;

        Ok(config)
    }

    pub fn load_from_file(file: &File) -> Result<Self, ConfigError> {
        let config = serde_yaml::from_reader(file)?;

        Ok(config)
    }

    fn save_default_config<P: AsRef<Path>>(config_path: P) -> Result<(), ConfigError> {
        let config_path = config_path.as_ref();

        if let Some(config_dir) = config_path.parent() {
            create_dir_all(config_dir)?;
        }

        let mut config_file = File::create(config_path)?;
        config_file.write_all(DEFAULT_CONFIG_STR.as_bytes())?;

        Ok(())
    }

    pub fn load_from_file_or_create_default<P: AsRef<Path>>(
        config_path: P,
    ) -> Result<Config, ConfigError> {
        if let Ok(config_file) = File::open(&config_path) {
            let config = Self::load_from_file(&config_file)?;

            Ok(config)
        } else {
            // No config file
            let _ = Self::save_default_config(&config_path); // Ignore error
            let config = Self::load_from_str(&DEFAULT_CONFIG_STR)?;

            Ok(config)
        }
    }

    pub fn config_path() -> PathBuf {
        if let Some(path) = std::env::var_os("ALMEL_CONFIG_PATH").map(PathBuf::from) {
            path
        } else if let Some(config_home) = std::env::var_os("XDG_CONFIG_HOME").map(PathBuf::from) {
            let mut path = config_home;
            path.push("almel/almel.yaml");

            path
        } else if let Some(home) = dirs::home_dir() {
            let mut path = home;
            path.push(".config/almel/almel.yaml");

            path
        } else {
            PathBuf::from("/etc/almel/almel.yaml")
        }
    }
}

lazy_static! {
    static ref DEFAULT_CONFIG_STR: &'static str = include_str!("almel.yaml");
}
