mod status_git2;

use crate::configs::git_repo::ConfigIcons;
use crate::context::Context;
use crate::segments::Segment;
use std::cmp::min;
use std::fmt::Write as _;

#[derive(Debug)]
pub enum HeadStatus {
    Branch(String),
    Tag(String),
    Commit(String),
}

fn build_head_status_text(
    head_status: &HeadStatus,
    icons: &ConfigIcons,
    display_master: bool,
    commit_hash_len: usize,
) -> String {
    match head_status {
        HeadStatus::Branch(name) if !display_master && (name == "master" || name == "main") => {
            icons.branch.to_string()
        }
        HeadStatus::Branch(name) => format!("{} {}", icons.branch, name),
        HeadStatus::Tag(name) => format!("{} {}", icons.tag, name),
        HeadStatus::Commit(hash) => {
            let shorten_hash = &hash[..min(commit_hash_len, hash.len())];
            format!("{} {}", icons.commit, shorten_hash)
        }
    }
}

#[test]
fn test_build_head_status_text() {
    let icons = ConfigIcons {
        branch: "B".to_string(),
        tag: "T".to_string(),
        commit: "C".to_string(),
        modified: "M".to_string(),
        added: "+".to_string(),
        deleted: "-".to_string(),
        added_deleted: "±".to_string(),
        conflicted: "C".to_string(),
        behind: "↓".to_string(),
        ahead: "↑".to_string(),
    };

    struct Scenario {
        status: HeadStatus,
        display_master: bool,
        commit_hash_len: usize,
        expected: &'static str,
    }

    let scenarios = &[
        Scenario {
            status: HeadStatus::Branch("master".to_string()),
            display_master: true,
            commit_hash_len: 7,
            expected: "B master",
        },
        Scenario {
            status: HeadStatus::Branch("master".to_string()),
            display_master: false,
            commit_hash_len: 7,
            expected: "B",
        },
        Scenario {
            status: HeadStatus::Branch("main".to_string()),
            display_master: false,
            commit_hash_len: 7,
            expected: "B",
        },
        Scenario {
            status: HeadStatus::Branch("some-branch".to_string()),
            display_master: true,
            commit_hash_len: 7,
            expected: "B some-branch",
        },
        Scenario {
            status: HeadStatus::Branch("some-branch".to_string()),
            display_master: false,
            commit_hash_len: 7,
            expected: "B some-branch",
        },
        Scenario {
            status: HeadStatus::Tag("some-tag".to_string()),
            display_master: false,
            commit_hash_len: 7,
            expected: "T some-tag",
        },
        Scenario {
            status: HeadStatus::Commit("0123456789ABCEDF".to_string()),
            display_master: false,
            commit_hash_len: 7,
            expected: "C 0123456",
        },
        Scenario {
            status: HeadStatus::Commit("0123456789ABCEDF".to_string()),
            display_master: false,
            commit_hash_len: 5,
            expected: "C 01234",
        },
        Scenario {
            status: HeadStatus::Commit("0123456789ABCEDF".to_string()),
            display_master: false,
            commit_hash_len: 999,
            expected: "C 0123456789ABCEDF",
        },
    ];

    for s in scenarios {
        let actual = build_head_status_text(&s.status, &icons, s.display_master, s.commit_hash_len);
        assert_eq!(actual, s.expected);
    }
}

#[derive(Debug)]
pub struct WorktreeStatus {
    num_unstaged_new: u32,
    num_unstaged_delete: u32,
    num_unstaged_changes: u32,
    num_staged_new: u32,
    num_staged_delete: u32,
    num_staged_changes: u32,
    num_conflicted: u32,
}
impl WorktreeStatus {
    pub fn is_conflicted(&self) -> bool {
        self.num_conflicted > 0
    }
    pub fn is_modified(&self) -> bool {
        self.num_unstaged_changes > 0 || self.num_staged_changes > 0
    }
    pub fn has_added(&self) -> bool {
        self.num_unstaged_new > 0 || self.num_staged_new > 0
    }
    pub fn has_deleted(&self) -> bool {
        self.num_unstaged_delete > 0 || self.num_staged_delete > 0
    }
    pub fn has_unstaged_changes(&self) -> bool {
        self.num_unstaged_new > 0 || self.num_unstaged_delete > 0 || self.num_unstaged_changes > 0
    }
    pub fn has_staged_changes(&self) -> bool {
        self.num_staged_new > 0 || self.num_staged_delete > 0 || self.num_staged_changes > 0
    }
}

fn build_worktree_status_text(worktree_status: &WorktreeStatus, icons: &ConfigIcons) -> String {
    let mut text = String::new();

    if worktree_status.is_modified() {
        text += &icons.modified;
    }

    match (worktree_status.has_added(), worktree_status.has_deleted()) {
        (true, true) => text += &icons.added_deleted,
        (true, false) => text += &icons.added,
        (false, true) => text += &icons.deleted,
        (false, false) => {}
    }

    text
}

#[test]
fn test_build_worktree_status_text() {
    let icons = ConfigIcons {
        branch: "B".to_string(),
        tag: "T".to_string(),
        commit: "C".to_string(),
        modified: "M".to_string(),
        added: "+".to_string(),
        deleted: "-".to_string(),
        added_deleted: "±".to_string(),
        conflicted: "C".to_string(),
        behind: "↓".to_string(),
        ahead: "↑".to_string(),
    };

    struct Scenario {
        status: WorktreeStatus,
        expected: &'static str,
    }

    let scenarios = &[
        Scenario {
            status: WorktreeStatus {
                num_unstaged_new: 0,
                num_unstaged_delete: 0,
                num_unstaged_changes: 0,
                num_staged_new: 0,
                num_staged_delete: 0,
                num_staged_changes: 0,
                num_conflicted: 0,
            },
            expected: "",
        },
        Scenario {
            status: WorktreeStatus {
                num_unstaged_new: 0,
                num_unstaged_delete: 0,
                num_unstaged_changes: 1,
                num_staged_new: 0,
                num_staged_delete: 0,
                num_staged_changes: 0,
                num_conflicted: 0,
            },
            expected: "M",
        },
        Scenario {
            status: WorktreeStatus {
                num_unstaged_new: 1,
                num_unstaged_delete: 0,
                num_unstaged_changes: 0,
                num_staged_new: 0,
                num_staged_delete: 0,
                num_staged_changes: 1,
                num_conflicted: 0,
            },
            expected: "M+",
        },
        Scenario {
            status: WorktreeStatus {
                num_unstaged_new: 0,
                num_unstaged_delete: 0,
                num_unstaged_changes: 0,
                num_staged_new: 1,
                num_staged_delete: 0,
                num_staged_changes: 1,
                num_conflicted: 0,
            },
            expected: "M+",
        },
        Scenario {
            status: WorktreeStatus {
                num_unstaged_new: 1,
                num_unstaged_delete: 0,
                num_unstaged_changes: 0,
                num_staged_new: 0,
                num_staged_delete: 1,
                num_staged_changes: 0,
                num_conflicted: 0,
            },
            expected: "±",
        },
    ];

    for s in scenarios {
        let actual = build_worktree_status_text(&s.status, &icons);
        assert_eq!(actual, s.expected);
    }
}

#[derive(Debug)]
pub struct RemoteStatus {
    commits_behind: u32,
    commits_ahead: u32,
}

fn build_remote_status_text(remote_status: &RemoteStatus, icons: &ConfigIcons) -> String {
    let mut text = String::new();

    if remote_status.commits_behind != 0 {
        let _ = write!(text, "{}{}", icons.behind, remote_status.commits_behind);
    }
    if remote_status.commits_ahead != 0 {
        let _ = write!(text, "{}{}", icons.ahead, remote_status.commits_ahead);
    }

    text
}

#[test]
fn test_build_remote_status_text() {
    let icons = ConfigIcons {
        branch: "B".to_string(),
        tag: "T".to_string(),
        commit: "C".to_string(),
        modified: "M".to_string(),
        added: "+".to_string(),
        deleted: "-".to_string(),
        added_deleted: "±".to_string(),
        conflicted: "C".to_string(),
        behind: "↓".to_string(),
        ahead: "↑".to_string(),
    };

    struct Scenario {
        status: RemoteStatus,
        expected: &'static str,
    }

    let scenarios = &[
        Scenario {
            status: RemoteStatus {
                commits_behind: 0,
                commits_ahead: 0,
            },
            expected: "",
        },
        Scenario {
            status: RemoteStatus {
                commits_behind: 1,
                commits_ahead: 0,
            },
            expected: "↓1",
        },
        Scenario {
            status: RemoteStatus {
                commits_behind: 0,
                commits_ahead: 2,
            },
            expected: "↑2",
        },
        Scenario {
            status: RemoteStatus {
                commits_behind: 3,
                commits_ahead: 4,
            },
            expected: "↓3↑4",
        },
    ];

    for s in scenarios {
        let actual = build_remote_status_text(&s.status, &icons);
        assert_eq!(actual, s.expected);
    }
}

#[derive(Debug)]
pub struct RepoStatus {
    pub head_status: HeadStatus,
    pub worktree_status: WorktreeStatus,
    pub remote_status: Option<RemoteStatus>,
}

pub fn build_segment<'ctx>(context: &'ctx Context) -> Option<Segment<'ctx>> {
    let config = &context.config.git_repo;
    let icons = &config.icons;

    let status = status_git2::git2_repo_status(context)?;

    // Build content
    let mut content = String::new();

    let head_text = build_head_status_text(
        &status.head_status,
        icons,
        config.display_master,
        config.commit_hash_len,
    );
    content += &head_text;

    let worktree_text = build_worktree_status_text(&status.worktree_status, icons);
    if !worktree_text.is_empty() {
        content += " ";
        content += &worktree_text;
    }

    let remote_text = status
        .remote_status
        .as_ref()
        .map(|s| build_remote_status_text(s, icons))
        .unwrap_or_default();

    if !remote_text.is_empty() {
        content += " ";
        content += &remote_text;
    }

    let style = if status.worktree_status.is_conflicted() {
        &config.conflicted.style
    } else if status.worktree_status.has_unstaged_changes() {
        &config.unstaged.style
    } else if status.worktree_status.has_staged_changes() {
        &config.staged.style
    } else {
        &config.clean.style
    };

    Some(Segment { style, content })
}
