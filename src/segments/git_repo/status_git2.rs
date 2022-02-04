use git2::{BranchType, Oid, Reference, Repository, Status, StatusOptions};

use super::{HeadStatus, RemoteStatus, RepoStatus, WorktreeStatus};
use crate::context::Context;

fn find_tag<'a>(repo: &'a Repository, oid: &Oid) -> Option<Reference<'a>> {
    let references = repo.references().ok()?;

    references
        .flatten()
        .find(|r| r.is_tag() && r.target().as_ref() == Some(oid))
}

fn head_status(repo: &Repository, head: &Option<Reference>, display_tag: bool) -> HeadStatus {
    let head = match head {
        Some(head) => head,

        // Empty repository
        None => {
            let config = repo.config().ok();
            let default_branch = config.and_then(|c| c.get_string("init.defaultBranch").ok());
            return HeadStatus::Branch(default_branch.unwrap_or_else(|| "master".to_string()));
        }
    };

    if head.is_branch() {
        // HEAD is a branch
        return HeadStatus::Branch(head.shorthand().unwrap_or("?").to_string());
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
    HeadStatus::Commit(oid.to_string())
}

fn worktree_status(repo: &Repository) -> WorktreeStatus {
    let mut options = StatusOptions::new();
    options.include_untracked(true);

    let mut status = WorktreeStatus {
        num_unstaged_new: 0,
        num_unstaged_delete: 0,
        num_unstaged_changes: 0,
        num_staged_new: 0,
        num_staged_delete: 0,
        num_staged_changes: 0,
        num_conflicted: 0,
    };

    if let Ok(statuses) = repo.statuses(Some(&mut options)) {
        for s in statuses.iter() {
            let st = s.status();
            if st.intersects(Status::WT_NEW | Status::WT_RENAMED) {
                status.num_unstaged_new += 1;
            }
            if st.intersects(Status::WT_DELETED | Status::WT_RENAMED) {
                status.num_unstaged_delete += 1;
            }
            if st.intersects(Status::WT_MODIFIED | Status::WT_TYPECHANGE) {
                status.num_unstaged_changes += 1;
            }
            if st.intersects(Status::INDEX_NEW | Status::INDEX_RENAMED) {
                status.num_staged_new += 1;
            }
            if st.intersects(Status::INDEX_DELETED | Status::INDEX_RENAMED) {
                status.num_staged_delete += 1;
            }
            if st.intersects(Status::INDEX_MODIFIED | Status::INDEX_TYPECHANGE) {
                status.num_staged_changes += 1;
            }
            if st.intersects(Status::CONFLICTED) {
                status.num_conflicted += 1;
            }
        }
    }

    status
}

fn remote_status(repo: &Repository, head: &Option<Reference>) -> Option<RemoteStatus> {
    let head = head.as_ref()?;
    let branch_name = head.shorthand()?;

    let local_branch = repo.find_branch(branch_name, BranchType::Local).ok()?;
    let upstream_branch = local_branch.upstream().ok()?;

    let local_oid = head.target()?;
    let upstream_oid = upstream_branch.get().target()?;

    let (ahead, behind) = repo.graph_ahead_behind(local_oid, upstream_oid).ok()?;

    Some(RemoteStatus {
        commits_behind: behind as u32,
        commits_ahead: ahead as u32,
    })
}

pub fn git2_repo_status(context: &Context) -> Option<RepoStatus> {
    let config = &context.config.git_repo;

    let repo = context.git_repo.as_ref()?;
    let head = repo.head().ok();

    let head_status = head_status(repo, &head, config.display_tag);
    let worktree_status = worktree_status(repo);
    let remote_status = remote_status(repo, &head);

    Some(RepoStatus {
        head_status,
        worktree_status,
        remote_status,
    })
}
