use git2::{BranchType, Oid, Reference, Repository, Status, StatusOptions};
use std::io::Write;

use crate::config::GitHeadStatusConfig;
use crate::prompt::{Prompt, PromptError};

fn get_repository_statuses(repo: &Repository) -> Status {
    let mut status_options = StatusOptions::new();
    status_options.include_untracked(true);

    repo.statuses(Some(&mut status_options))
        .map(|statuses| statuses.iter().fold(Status::empty(), |a, b| a | b.status()))
        .unwrap_or(Status::empty())
}

fn get_segment_colors<'config>(
    config: &'config GitHeadStatusConfig,
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

fn write_head(
    segment: &mut String,
    config: &GitHeadStatusConfig,
    repo: &Repository,
    head: Option<&Reference>,
) {
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
    hash_str.truncate(config.commit_hash_len as usize);

    *segment += &format!("{} {}", commit_icon, hash_str);
}

fn write_status_icons(segment: &mut String, config: &GitHeadStatusConfig, statuses: &Status) {
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
    config: &GitHeadStatusConfig,
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
    config: &GitHeadStatusConfig,
    repo: &Repository,
    head: Option<&Reference>,
) {
    let _ = try_write_remote_status(segment, config, repo, head);
}

pub fn prompt_subsegment<W: Write>(
    p: &mut Prompt<W>,
    config: &GitHeadStatusConfig,
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
