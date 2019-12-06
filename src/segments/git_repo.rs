use ansi_term::Color;
use git2::{BranchType, Oid, Reference, Repository, Status, StatusOptions};
use lazy_static::lazy_static;

use crate::configs::git_repo::Config;
use crate::context::Context;
use crate::segments::Segment;

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
        .unwrap_or_else(|_| Status::empty())
}

fn find_tag<'a>(repo: &'a Repository, oid: &Oid) -> Option<Reference<'a>> {
    let references = repo.references().ok()?;

    references
        .flatten()
        .find(|r| r.is_tag() && r.target().as_ref() == Some(oid))
}

#[derive(Debug)]
enum HeadStatus<'a> {
    Branch(&'a str),
    Tag(String),
    Commit(String),
}

fn get_head_status<'a>(
    repo: &'a Repository,
    head: &'a Option<Reference>,
    display_tag: bool,
    commit_hash_len: usize,
) -> HeadStatus<'a> {
    let head = match head {
        Some(head) => head,

        // Empty repository
        None => return HeadStatus::Branch("master"),
    };

    if head.is_branch() {
        // HEAD is a branch
        return HeadStatus::Branch(head.shorthand().unwrap_or("?"));
    }

    let oid = match head.target() {
        Some(oid) => oid,

        // Because WTF?
        None => return HeadStatus::Commit("?".to_string()),
    };

    if display_tag {
        if let Some(tag) = find_tag(repo, &oid) {
            return HeadStatus::Tag(tag.shorthand().unwrap_or("?").to_string());
        }
    }

    // HEAD is a simple commit
    let mut hash_str = oid.to_string();
    hash_str.truncate(commit_hash_len);

    HeadStatus::Commit(hash_str)
}

fn build_status_icons(config: &Config, statuses: Status) -> String {
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
    config: &Config,
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

fn get_colors(config: &Config, statuses: Status) -> (Color, Color) {
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

pub fn build_segment(context: &Context) -> Option<Segment> {
    let config = &context.config.git_repo;

    let repo = context.git_repo.as_ref()?;
    let head = repo.head().ok();
    let statuses = get_repo_statuses(repo);

    let head_status = get_head_status(repo, &head, config.display_tag, config.commit_hash_len);
    let status_icons = build_status_icons(config, statuses);
    let remote_status = build_remote_status(config, &repo, &head);

    // Build content
    let mut content = String::new();

    match &head_status {
        HeadStatus::Branch("master") if !config.display_master => content += &config.icons.branch,
        HeadStatus::Branch(name) => content += &format!("{} {}", config.icons.branch, name),
        HeadStatus::Tag(name) => content += &format!("{} {}", config.icons.tag, name),
        HeadStatus::Commit(hash) => content += &format!("{} {}", config.icons.commit, hash),
    }

    if !status_icons.is_empty() {
        content += &format!(" {}", status_icons);
    }

    if let Some(remote_status) = remote_status {
        content += &format!(" {}", remote_status);
    }

    let (background, foreground) = get_colors(config, statuses);

    Some(Segment {
        background,
        foreground,
        content,
    })
}
