use git2::{Oid, Repository};
use std::io;

use crate::prompt::{Prompt, PromptError};

fn find_tag(repo: &Repository, oid: &Oid) -> Option<String> {
    let tag = repo
        .references()
        .ok()?
        .flatten()
        .filter(|r| r.is_tag())
        .filter(|r| r.target().as_ref() == Some(oid))
        .next()?;

    let tag_name = tag.shorthand()?.to_string();

    Some(tag_name.to_string())
}

pub fn prompt_segment<W: io::Write>(p: &mut Prompt<W>) -> Result<(), PromptError> {
    // Open the current repository
    let repo = match Repository::discover(".") {
        Ok(repo) => repo,
        Err(_) => return Ok(()),
    };

    // Get HEAD
    let head = match repo.head() {
        Ok(head) => head,
        Err(_) => return Ok(()), // TODO: Repositoty without any commits
    };

    // Show the HEAD status
    let head_text;

    if head.is_branch() {
        // Show the current branch name
        let branch_name = head.shorthand().unwrap_or("?");
        let branch_icon = "\u{f418}";

        head_text = format!("{} {}", branch_icon, branch_name);
    } else if let Some(oid) = head.target() {
        if let Some(tag) = find_tag(&repo, &oid) {
            // Show the tag name
            let tag_icon = "\u{f412}";

            head_text = format!("{} {}", tag_icon, tag);
        } else {
            // Show the current commit hash
            let hash_len = 6;
            let mut hash = oid.to_string();
            hash.truncate(hash_len);

            let commit_icon = "\u{f417}";

            head_text = format!("{} {}", commit_icon, hash);
        }
    } else {
        head_text = "?".to_string();
    }

    p.write_segment("green", "black", &head_text)?;

    // Show the user name
    if let Ok(config) = repo.config() {
        if let Ok(user_name) = config.get_string("user.name") {
            p.write_segment("cyan", "black", &format!("{} {}", "\u{f2c0}", user_name))?;
        }
    }

    Ok(())
}
