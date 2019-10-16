use git2::{Oid, Repository, Status, StatusOptions};
use std::io;

use crate::prompt::{Prompt, PromptError};

#[derive(Debug)]
pub enum HeadStatus {
    Branch(String),
    Tag(String),
    Commit(Oid),
    Unknown,
}

fn find_tag(repo: &Repository, oid: &Oid) -> Option<Option<String>> {
    let tag = repo
        .references()
        .ok()?
        .flatten()
        .filter(|r| r.is_tag())
        .filter(|r| r.target().as_ref() == Some(oid))
        .next()?;

    Some(tag.shorthand().map(|name| name.to_string()))
}

pub fn get_head_status(repo: &Repository) -> Option<HeadStatus> {
    let default_ref_name = "???";
    let head = repo.head().ok()?;

    if head.is_branch() {
        // HEAD is a branch
        let branch_name = head.shorthand().unwrap_or(default_ref_name).to_string();

        Some(HeadStatus::Branch(branch_name))
    } else if let Some(oid) = head.target() {
        if let Some(tag_name) = find_tag(repo, &oid) {
            // HEAD is a tag
            let tag_name = tag_name.unwrap_or(default_ref_name.to_string());

            Some(HeadStatus::Tag(tag_name))
        } else {
            // HEAD is a commit
            Some(HeadStatus::Commit(oid))
        }
    } else {
        Some(HeadStatus::Unknown)
    }
}

pub fn prompt_segment<W: io::Write>(p: &mut Prompt<W>) -> Result<(), PromptError> {
    // Open the current repository
    let repo = match Repository::discover(".") {
        Ok(repo) => repo,
        Err(_) => return Ok(()),
    };

    // Show the HEAD status
    let branch_icon = "\u{f418}";
    let tag_icon = "\u{f412}";
    let hash_len = 6;
    let commit_icon = "\u{f417}";

    let head_text = match get_head_status(&repo) {
        Some(HeadStatus::Branch(name)) => format!("{} {}", branch_icon, name),
        Some(HeadStatus::Tag(name)) => format!("{} {}", tag_icon, name),
        Some(HeadStatus::Commit(oid)) => {
            let mut hash_str = oid.to_string();
            hash_str.truncate(hash_len);
            format!("{} {}", commit_icon, hash_str)
        }
        Some(HeadStatus::Unknown) | None => "?".to_string(),
    };

    // Get statuses
    let mut file_statuses = Status::empty();

    if let Ok(statuses) = repo.statuses(Some(StatusOptions::new().include_untracked(true))) {
        for status in statuses.iter() {
            file_statuses.insert(status.status());

            if file_statuses.is_all() {
                break;
            }
        }
    }

    // Set colors
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

    let background;
    let foreground;

    if file_statuses.intersects(conflicted_statuses) {
        background = "red";
        foreground = "black";
    } else if file_statuses.intersects(unstaged_statuses) {
        background = "yellow";
        foreground = "black";
    } else if file_statuses.intersects(staged_statuses) {
        background = "green";
        foreground = "black";
    } else {
        background = "green";
        foreground = "black";
    }

    // Get status icons
    let modified_statuses = Status::INDEX_MODIFIED
        | Status::INDEX_RENAMED
        | Status::INDEX_TYPECHANGE
        | Status::WT_MODIFIED
        | Status::WT_RENAMED
        | Status::WT_TYPECHANGE;
    let added_statuses = Status::INDEX_NEW | Status::WT_NEW;
    let deleted_statuses = Status::INDEX_DELETED | Status::WT_DELETED;

    let modified_icon = "…";
    let plus_minus_icon = "±";
    let plus_icon = "+";
    let minus_icon = "-";
    let conflict_icon = "\u{f47f}";

    let mut icons = String::new();

    if file_statuses.intersects(modified_statuses) {
        icons += modified_icon;
    }

    match (
        file_statuses.intersects(added_statuses),
        file_statuses.intersects(deleted_statuses),
    ) {
        (true, true) => icons += plus_minus_icon,
        (true, false) => icons += plus_icon,
        (false, true) => icons += minus_icon,
        (false, false) => {}
    }

    if file_statuses.intersects(conflicted_statuses) {
        icons += conflict_icon;
    }

    p.write_segment(
        background,
        foreground,
        &if icons.is_empty() {
            format!("{}", head_text)
        } else {
            format!("{} {}", head_text, icons)
        },
    )?;

    // Show the user name
    if let Ok(config) = repo.config() {
        if let Ok(user_name) = config.get_string("user.name") {
            p.write_segment("cyan", "black", &format!("{} {}", "\u{f2c0}", user_name))?;
        }
    }

    Ok(())
}
