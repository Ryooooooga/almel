use git2::{BranchType, Oid, Reference, Repository, Status, StatusOptions};
use lazy_static::lazy_static;

use crate::color::Color;
use crate::config::GitRepoConfig;
use crate::context::Context;
use crate::segments::{Segment, SegmentError};

lazy_static! {
    static ref STATUS_CONFLICTED: Status = Status::CONFLICTED;
    static ref STATUS_UNSTAGED: Status = Status::WT_NEW
        | Status::WT_MODIFIED
        | Status::WT_DELETED
        | Status::WT_RENAMED
        | Status::WT_TYPECHANGE;
    static ref STATUS_STAGED: Status = Status::INDEX_NEW
        | Status::INDEX_MODIFIED
        | Status::INDEX_DELETED
        | Status::INDEX_RENAMED
        | Status::INDEX_TYPECHANGE;
    static ref STATUS_MODIFIED: Status = Status::INDEX_MODIFIED
        | Status::INDEX_RENAMED
        | Status::INDEX_TYPECHANGE
        | Status::WT_MODIFIED
        | Status::WT_RENAMED
        | Status::WT_TYPECHANGE;
    static ref STATUS_NEW: Status = Status::WT_NEW | Status::INDEX_NEW;
    static ref STATUS_DELETED: Status = Status::WT_DELETED | Status::INDEX_DELETED;
}

fn get_repo_statuses(repo: &Repository) -> Status {
    let mut options = StatusOptions::new();
    options.include_untracked(true);

    repo.statuses(Some(&mut options))
        .map(|statuses| statuses.iter().fold(Status::empty(), |a, b| a | b.status()))
        .unwrap_or(Status::empty())
}

fn find_tag<'a>(repo: &'a Repository, oid: &Oid) -> Option<Reference<'a>> {
    let references = repo.references().ok()?;

    references
        .flatten()
        .filter(|r| r.is_tag())
        .filter(|r| r.target().as_ref() == Some(oid))
        .next()
}

fn build_head_name(config: &GitRepoConfig, repo: &Repository, head: &Option<Reference>) -> String {
    let head = match head {
        Some(head) => head,

        // Empty repository
        None => return format!("{} {}", config.icons.branch, "master"),
    };

    if head.is_branch() {
        // HEAD is a branch
        return format!(
            "{} {}",
            config.icons.branch,
            head.shorthand().unwrap_or("?")
        );
    }

    let oid = match head.target() {
        Some(oid) => oid,

        // Because WTF?
        None => return format!("{} {}", config.icons.commit, "?"),
    };

    if config.display_tag {
        if let Some(tag) = find_tag(repo, &oid) {
            return format!("{} {}", config.icons.tag, tag.shorthand().unwrap_or("?"));
        }
    }

    // HEAD is a simple commit
    let mut hash_str = oid.to_string();
    hash_str.truncate(config.commit_hash_len);

    format!("{} {}", config.icons.commit, hash_str)
}

fn build_status_icons(config: &GitRepoConfig, statuses: &Status) -> String {
    let mut icons = String::new();

    if statuses.intersects(*STATUS_MODIFIED) {
        icons += &config.icons.modified;
    }

    match (
        statuses.intersects(*STATUS_NEW),
        statuses.intersects(*STATUS_DELETED),
    ) {
        (true, true) => icons += &config.icons.added_deleted,
        (true, false) => icons += &config.icons.added,
        (false, true) => icons += &config.icons.deleted,
        (false, false) => {}
    }

    icons
}

fn build_remote_status<'a>(
    config: &GitRepoConfig,
    repo: &'a Repository,
    head: &Option<Reference<'a>>,
) -> Option<String> {
    let head = head.as_ref()?;
    let branch_name = head.shorthand()?;

    let local_branch = repo.find_branch(branch_name, BranchType::Local).ok()?;
    let upstream_branch = local_branch.upstream().ok()?;

    let local_oid = head.target()?;
    let upstream_oid = upstream_branch.get().target()?;

    let (ahead, behind) = repo.graph_ahead_behind(local_oid, upstream_oid).ok()?;

    if (ahead, behind) == (0, 0) {
        return None;
    }

    let mut status = String::new();

    if behind != 0 {
        status += &format!("{}{}", config.icons.behind, behind);
    }
    if ahead != 0 {
        status += &format!("{}{}", config.icons.ahead, ahead);
    }

    Some(status)
}

fn get_colors(config: &GitRepoConfig, statuses: &Status) -> (Color, Color) {
    if statuses.intersects(*STATUS_CONFLICTED) {
        (config.conflicted.background, config.conflicted.foreground)
    } else if statuses.intersects(*STATUS_UNSTAGED) {
        (config.unstaged.background, config.unstaged.foreground)
    } else if statuses.intersects(*STATUS_STAGED) {
        (config.staged.background, config.staged.foreground)
    } else {
        (config.clean.background, config.clean.foreground)
    }
}

pub fn build_segment(context: &Context) -> Result<Option<Segment>, SegmentError> {
    let config = &context.config.git_repo;

    let repo = match &context.git_repo {
        Some(repo) => repo,
        None => return Ok(None),
    };

    let head = repo.head().ok();
    let statuses = get_repo_statuses(repo);

    let head_name = build_head_name(config, repo, &head);
    let status_icons = build_status_icons(config, &statuses);
    let remote_status = build_remote_status(config, &repo, &head);

    // Build content
    let mut content = head_name;

    if !status_icons.is_empty() {
        content += &format!(" {}", status_icons);
    }

    if let Some(remote_status) = remote_status {
        content += &format!(" {}", remote_status);
    }

    let (background, foreground) = get_colors(config, &statuses);

    Ok(Some(Segment {
        background,
        foreground,
        content,
    }))
}
