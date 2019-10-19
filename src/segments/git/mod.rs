mod git_user;

use git2::{BranchType, Oid, Reference, Repository, Status, StatusOptions};
use std::io;

use crate::config::GitConfig;
use crate::prompt::{Prompt, PromptError};

fn find_tag<'repo>(repo: &'repo Repository, oid: &Oid) -> Option<Reference<'repo>> {
    repo.references()
        .ok()?
        .flatten()
        .filter(|r| r.is_tag())
        .filter(|r| r.target().as_ref() == Some(oid))
        .next()
}

fn format_head_status(repo: &Repository, head: &Option<Reference>) -> String {
    let branch_icon = "\u{f418}";
    let tag_icon = "\u{f412}";
    let hash_len = 6;
    let commit_icon = "\u{f417}";

    let head = match head {
        Some(head) => head,

        // Empty repository
        None => {
            return format!("{} {}", branch_icon, "master");
        }
    };

    if head.is_branch() {
        // HEAD is a branch
        let branch_name = head.shorthand().unwrap_or("?");

        return format!("{} {}", branch_icon, branch_name);
    }

    // Get the commit hash of HEAD
    let oid = match head.target() {
        Some(oid) => oid,

        // Because WTF
        None => {
            return format!("{} {}", branch_icon, "!");
        }
    };

    if let Some(tag) = find_tag(repo, &oid) {
        // HEAD is a tag
        let tag_name = tag.shorthand().unwrap_or("?");

        return format!("{} {}", tag_icon, tag_name);
    } else {
        // HEAD is a commit
        let mut hash_str = oid.to_string();
        hash_str.truncate(hash_len);

        return format!("{} {}", commit_icon, hash_str);
    }
}

fn get_repository_statuses(repo: &Repository) -> Status {
    let mut status_options = StatusOptions::new();
    status_options.include_untracked(true);

    repo.statuses(Some(&mut status_options))
        .map(|statuses| statuses.iter().fold(Status::empty(), |a, b| a | b.status()))
        .unwrap_or(Status::empty())
}

fn format_status_icons(repo_statuses: &Status) -> Option<String> {
    let modified_statuses = Status::INDEX_MODIFIED
        | Status::INDEX_RENAMED
        | Status::INDEX_TYPECHANGE
        | Status::WT_MODIFIED
        | Status::WT_RENAMED
        | Status::WT_TYPECHANGE;
    let added_statuses = Status::INDEX_NEW | Status::WT_NEW;
    let deleted_statuses = Status::INDEX_DELETED | Status::WT_DELETED;
    let conflicted_statuses = Status::CONFLICTED;

    let modified_icon = "…";
    let plus_minus_icon = "±";
    let plus_icon = "+";
    let minus_icon = "-";
    let conflict_icon = "\u{f47f}";

    let mut status_icons = String::new();

    if repo_statuses.intersects(modified_statuses) {
        status_icons += modified_icon;
    }

    match (
        repo_statuses.intersects(added_statuses),
        repo_statuses.intersects(deleted_statuses),
    ) {
        (true, true) => status_icons += plus_minus_icon,
        (true, false) => status_icons += plus_icon,
        (false, true) => status_icons += minus_icon,
        (false, false) => {}
    }

    if repo_statuses.intersects(conflicted_statuses) {
        status_icons += conflict_icon;
    }

    if status_icons.is_empty() {
        None
    } else {
        Some(status_icons)
    }
}

fn get_segment_colors(repo_statuses: &Status) -> (&'static str, &'static str) {
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

    if repo_statuses.intersects(conflicted_statuses) {
        ("red", "black")
    } else if repo_statuses.intersects(unstaged_statuses) {
        ("yellow", "black")
    } else if repo_statuses.intersects(staged_statuses) {
        ("green", "black")
    } else {
        ("green", "black")
    }
}

fn format_graph_icons(repo: &Repository, head: &Option<Reference>) -> Option<String> {
    let head = head.as_ref()?;

    let ahead_icon = "\u{f176}";
    let behind_icon = "\u{f175}";

    // Find the local branch
    let local_branch = repo
        .find_branch(head.shorthand()?, BranchType::Local)
        .ok()?;
    let upstream_branch = local_branch.upstream().ok()?;

    let head_oid = head.target()?;
    let upstream_oid = upstream_branch.get().target()?;

    let (ahead, behind) = repo.graph_ahead_behind(head_oid, upstream_oid).ok()?;

    match (ahead, behind) {
        (0, 0) => None,
        (ahead, 0) => Some(format!("{}{}", ahead_icon, ahead)),
        (0, behind) => Some(format!("{}{}", behind_icon, behind)),
        (ahead, behind) => Some(format!("{}{}{}{}", ahead_icon, ahead, behind_icon, behind)),
    }
}

pub fn prompt_segment<W: io::Write>(
    p: &mut Prompt<W>,
    config: &GitConfig,
) -> Result<(), PromptError> {
    // Open the current repository
    let repo = match Repository::discover(".") {
        Ok(repo) => repo,
        Err(_) => return Ok(()),
    };

    let head = repo.head().ok();

    // Show the HEAD status
    let head_status = format_head_status(&repo, &head);

    // Get statuses
    let repo_statuses = get_repository_statuses(&repo);

    // Show the status icons
    let status_icons = format_status_icons(&repo_statuses);

    // Get the segment colors
    let (background, foreground) = get_segment_colors(&repo_statuses);

    // Show the graph icons
    let graph_icons = format_graph_icons(&repo, &head);

    // Build the git segment
    let mut text = head_status;

    if let Some(status_icons) = status_icons {
        text += " ";
        text += &status_icons;
    }

    if let Some(graph_icons) = graph_icons {
        text += " ";
        text += &graph_icons;
    }

    p.write_segment(background, foreground, &text)?;

    // Show the user name
    git_user::prompt_subsegment(p, &config.user, &repo)?;

    Ok(())
}
