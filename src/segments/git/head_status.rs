use git2::{BranchType, Oid, Reference, Repository, Status, StatusOptions};
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::io::Write;

use crate::prompt::{Prompt, PromptError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "Config::default_display")]
    pub display: bool,

    #[serde(default)]
    pub clean: ConfigClean,
    #[serde(default)]
    pub unstaged: ConfigUnstaged,
    #[serde(default)]
    pub staged: ConfigStaged,
    #[serde(default)]
    pub conflicted: ConfigConflicted,

    #[serde(default = "Config::default_branch_icon")]
    pub branch_icon: String,
    #[serde(default = "Config::default_tag_icon")]
    pub tag_icon: String,
    #[serde(default = "Config::default_commit_icon")]
    pub commit_icon: String,
    #[serde(default = "Config::default_commit_hash_len")]
    pub commit_hash_len: usize,

    #[serde(default = "Config::default_modified_icon")]
    pub modified_icon: String,
    #[serde(default = "Config::default_added_icon")]
    pub added_icon: String,
    #[serde(default = "Config::default_deleted_icon")]
    pub deleted_icon: String,
    #[serde(default = "Config::default_added_deleted_icon")]
    pub added_deleted_icon: String,
    #[serde(default = "Config::default_conflicted_icon")]
    pub conflicted_icon: String,

    #[serde(default = "Config::default_behind_icon")]
    pub behind_icon: String,
    #[serde(default = "Config::default_ahead_icon")]
    pub ahead_icon: String,
}

impl Config {
    fn default_display() -> bool {
        true
    }
    fn default_branch_icon() -> String {
        "\u{f418}".to_string() // nf-oct-git_branch
    }
    fn default_tag_icon() -> String {
        "\u{f412}".to_string() // nf-oct-tag
    }
    fn default_commit_icon() -> String {
        "\u{f417}".to_string() // nf-oct-git_commit
    }
    fn default_commit_hash_len() -> usize {
        6
    }
    fn default_modified_icon() -> String {
        "…".to_string()
    }
    fn default_added_icon() -> String {
        "+".to_string()
    }

    fn default_deleted_icon() -> String {
        "-".to_string()
    }
    fn default_added_deleted_icon() -> String {
        "±".to_string()
    }
    fn default_conflicted_icon() -> String {
        "\u{f47f}".to_string() // nf-oct-git_compare
    }
    fn default_behind_icon() -> String {
        "\u{f175}".to_string() // nf-fa-long_arrow_down
    }
    fn default_ahead_icon() -> String {
        "\u{f176}".to_string() // nf-fa-long_arrow_up
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            display: Self::default_display(),

            clean: ConfigClean::default(),
            unstaged: ConfigUnstaged::default(),
            staged: ConfigStaged::default(),
            conflicted: ConfigConflicted::default(),

            branch_icon: Self::default_branch_icon(),
            tag_icon: Self::default_tag_icon(),
            commit_icon: Self::default_commit_icon(),
            commit_hash_len: Self::default_commit_hash_len(),

            modified_icon: Self::default_modified_icon(),
            added_icon: Self::default_added_icon(),
            deleted_icon: Self::default_deleted_icon(),
            added_deleted_icon: Self::default_added_deleted_icon(),
            conflicted_icon: Self::default_conflicted_icon(),

            behind_icon: Self::default_behind_icon(),
            ahead_icon: Self::default_ahead_icon(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigClean {
    #[serde(default = "ConfigClean::default_background")]
    pub background: String,

    #[serde(default = "ConfigClean::default_foreground")]
    pub foreground: String,
}

impl ConfigClean {
    fn default_background() -> String {
        "green".to_string()
    }

    fn default_foreground() -> String {
        "black".to_string()
    }
}

impl Default for ConfigClean {
    fn default() -> Self {
        Self {
            background: Self::default_background(),
            foreground: Self::default_foreground(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigUnstaged {
    #[serde(default = "ConfigUnstaged::default_background")]
    pub background: String,

    #[serde(default = "ConfigUnstaged::default_foreground")]
    pub foreground: String,
}

impl ConfigUnstaged {
    fn default_background() -> String {
        "yellow".to_string()
    }

    fn default_foreground() -> String {
        "black".to_string()
    }
}

impl Default for ConfigUnstaged {
    fn default() -> Self {
        Self {
            background: Self::default_background(),
            foreground: Self::default_foreground(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigStaged {
    #[serde(default = "ConfigStaged::default_background")]
    pub background: String,

    #[serde(default = "ConfigStaged::default_foreground")]
    pub foreground: String,
}

impl ConfigStaged {
    fn default_background() -> String {
        "green".to_string()
    }

    fn default_foreground() -> String {
        "black".to_string()
    }
}

impl Default for ConfigStaged {
    fn default() -> Self {
        Self {
            background: Self::default_background(),
            foreground: Self::default_foreground(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigConflicted {
    #[serde(default = "ConfigConflicted::default_background")]
    pub background: String,

    #[serde(default = "ConfigConflicted::default_foreground")]
    pub foreground: String,
}

impl ConfigConflicted {
    fn default_background() -> String {
        "red".to_string()
    }

    fn default_foreground() -> String {
        "black".to_string()
    }
}

impl Default for ConfigConflicted {
    fn default() -> Self {
        Self {
            background: Self::default_background(),
            foreground: Self::default_foreground(),
        }
    }
}

fn get_repository_statuses(repo: &Repository) -> Status {
    let mut status_options = StatusOptions::new();
    status_options.include_untracked(true);

    repo.statuses(Some(&mut status_options))
        .map(|statuses| statuses.iter().fold(Status::empty(), |a, b| a | b.status()))
        .unwrap_or(Status::empty())
}

fn get_segment_colors<'config>(
    config: &'config Config,
    statuses: &Status,
) -> (&'config str, &'config str) {
    let conflicted_statuses = Status::CONFLICTED;

    let unstaged_statuses = Status::WT_NEW
        | Status::WT_MODIFIED
        | Status::WT_DELETED
        | Status::WT_RENAMED
        | Status::WT_TYPECHANGE;

    let staged_statuses = Status::INDEX_NEW
        | Status::INDEX_MODIFIED
        | Status::INDEX_DELETED
        | Status::INDEX_RENAMED
        | Status::INDEX_TYPECHANGE;

    if statuses.intersects(conflicted_statuses) {
        (&config.conflicted.background, &config.conflicted.foreground)
    } else if statuses.intersects(unstaged_statuses) {
        (&config.unstaged.background, &config.unstaged.foreground)
    } else if statuses.intersects(staged_statuses) {
        (&config.staged.background, &config.staged.foreground)
    } else {
        (&config.clean.background, &config.clean.foreground)
    }
}

fn find_tag<'repo>(repo: &'repo Repository, oid: &Oid) -> Option<Reference<'repo>> {
    repo.references()
        .ok()?
        .flatten()
        .filter(|r| r.is_tag())
        .filter(|r| r.target().as_ref() == Some(oid))
        .next()
}

fn write_head(segment: &mut String, config: &Config, repo: &Repository, head: Option<&Reference>) {
    let branch_icon = &config.branch_icon;
    let tag_icon = &config.tag_icon;
    let commit_icon = &config.commit_icon;

    let head = match head {
        Some(head) => head,

        // Empty repository
        None => {
            *segment += &format!("{} {}", branch_icon, "master");
            return;
        }
    };

    if head.is_branch() {
        // HEAD is a branch
        let branch_name = head.shorthand().unwrap_or("?");

        *segment += &format!("{} {}", branch_icon, branch_name);
        return;
    }

    // Get the commit hash of HEAD
    let oid = match head.target() {
        Some(oid) => oid,

        // Because WTF
        None => {
            *segment += &format!("{} {}", branch_icon, "!");
            return;
        }
    };

    if let Some(tag) = find_tag(repo, &oid) {
        // HEAD is a tag
        let tag_name = tag.shorthand().unwrap_or("?");

        *segment += &format!("{} {}", tag_icon, tag_name);
        return;
    }

    // HEAD is a commit
    let mut hash_str = oid.to_string();
    hash_str.truncate(config.commit_hash_len);

    *segment += &format!("{} {}", commit_icon, hash_str);
}

fn write_status_icons(segment: &mut String, config: &Config, statuses: &Status) {
    let modified_statuses = Status::INDEX_MODIFIED
        | Status::INDEX_RENAMED
        | Status::INDEX_TYPECHANGE
        | Status::WT_MODIFIED
        | Status::WT_RENAMED
        | Status::WT_TYPECHANGE;
    let added_statuses = Status::INDEX_NEW | Status::WT_NEW;
    let deleted_statuses = Status::INDEX_DELETED | Status::WT_DELETED;
    let conflicted_statuses = Status::CONFLICTED;

    let mut icons = String::new();

    if statuses.intersects(modified_statuses) {
        icons += &config.modified_icon;
    }

    match (
        statuses.intersects(added_statuses),
        statuses.intersects(deleted_statuses),
    ) {
        (true, true) => icons += &config.added_deleted_icon,
        (true, false) => icons += &config.added_icon,
        (false, true) => icons += &config.deleted_icon,
        (false, false) => {}
    }

    if statuses.intersects(conflicted_statuses) {
        icons += &config.conflicted_icon;
    }

    if !icons.is_empty() {
        *segment += " ";
        *segment += &icons;
    }
}

fn try_write_remote_status(
    segment: &mut String,
    config: &Config,
    repo: &Repository,
    head: Option<&Reference>,
) -> Option<()> {
    let head = head?;
    let branch_name = head.shorthand()?;

    // Find the local and remote branch
    let local_branch = repo.find_branch(branch_name, BranchType::Local).ok()?;
    let upstream_branch = local_branch.upstream().ok()?;

    let head_oid = head.target()?;
    let upstream_oid = upstream_branch.get().target()?;

    let (ahead, behind) = repo.graph_ahead_behind(head_oid, upstream_oid).ok()?;

    if ahead != 0 || behind != 0 {
        *segment += " ";
    }
    if ahead != 0 {
        *segment += &format!("{}{}", config.ahead_icon, ahead);
    }
    if behind != 0 {
        *segment += &format!("{}{}", config.behind_icon, behind);
    }

    Some(())
}

fn write_remote_status(
    segment: &mut String,
    config: &Config,
    repo: &Repository,
    head: Option<&Reference>,
) {
    let _ = try_write_remote_status(segment, config, repo, head);
}

pub fn prompt_subsegment<W: Write>(
    p: &mut Prompt<W>,
    config: &Config,
    repo: &Repository,
) -> Result<(), PromptError> {
    if !config.display {
        return Ok(());
    }

    let head = repo.head().ok();
    let head = head.as_ref();

    let statuses = get_repository_statuses(&repo);
    let (background, foreground) = get_segment_colors(config, &statuses);

    let mut segment = String::new();

    write_head(&mut segment, config, &repo, head);
    write_status_icons(&mut segment, config, &statuses);
    write_remote_status(&mut segment, config, &repo, head);

    p.write_segment(background, foreground, &segment)?;

    Ok(())
}
