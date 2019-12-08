use serde::{Deserialize, Serialize};
use std::default::Default;

use crate::configs::SegmentStyle;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub icons: ConfigIcons,

    #[serde(default)]
    pub clean: ConfigClean,

    #[serde(default)]
    pub unstaged: ConfigUnstaged,

    #[serde(default)]
    pub staged: ConfigStaged,

    #[serde(default)]
    pub conflicted: ConfigConflicted,

    #[serde(default = "Config::default_display_master")]
    pub display_master: bool,

    #[serde(default = "Config::default_display_tag")]
    pub display_tag: bool,

    #[serde(default = "Config::default_commit_hash_len")]
    pub commit_hash_len: usize,
}
impl Config {
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
impl Default for Config {
    fn default() -> Self {
        Self {
            icons: Default::default(),
            clean: Default::default(),
            unstaged: Default::default(),
            staged: Default::default(),
            conflicted: Default::default(),
            display_master: Self::default_display_master(),
            display_tag: Self::default_display_tag(),
            commit_hash_len: Self::default_commit_hash_len(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigIcons {
    #[serde(default = "ConfigIcons::default_branch")]
    pub branch: String,

    #[serde(default = "ConfigIcons::default_tag")]
    pub tag: String,

    #[serde(default = "ConfigIcons::default_commit")]
    pub commit: String,

    #[serde(default = "ConfigIcons::default_modified")]
    pub modified: String,

    #[serde(default = "ConfigIcons::default_added")]
    pub added: String,

    #[serde(default = "ConfigIcons::default_deleted")]
    pub deleted: String,

    #[serde(default = "ConfigIcons::default_added_deleted")]
    pub added_deleted: String,

    #[serde(default = "ConfigIcons::default_conflicted")]
    pub conflicted: String,

    #[serde(default = "ConfigIcons::default_behind")]
    pub behind: String,

    #[serde(default = "ConfigIcons::default_ahead")]
    pub ahead: String,
}
impl ConfigIcons {
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
impl Default for ConfigIcons {
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

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ConfigClean {
    #[serde(default)]
    pub style: SegmentStyle,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ConfigUnstaged {
    #[serde(default)]
    pub style: SegmentStyle,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ConfigStaged {
    #[serde(default)]
    pub style: SegmentStyle,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ConfigConflicted {
    #[serde(default)]
    pub style: SegmentStyle,
}
