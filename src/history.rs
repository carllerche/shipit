use crate::git::{self, Repository};
use crate::github;
use crate::util;
use git2;

use std::collections::VecDeque;

pub struct History {
    commits: Vec<()>,
}

pub struct Commit {
    sha: git2::Oid,
    pull: u64,
}

impl History {
    /// Load git history for a branch, including pull requests.
    pub fn load<T>(
        // The repository to load from
        repository: &mut Repository,

        // The git branch to traverse
        head: &git::Ref<T>,

        // Refs to traverse until
        terminals: &[git::Ref<T>],

        // Pull requests
        pulls: impl Iterator<Item = github::Pull>,
    ) -> History
    where
        T: AsRef<str>,
    {
        for terminal in terminals {
            assert!(repository.is_descendant_of(terminal, head));
        }

        let merge_base = repository.merge_base(terminals);

        // We will need to iterate the pull requests multiple times.
        let pulls = util::Replay::new(pulls);

        // Commits part of the history
        // let mut commits = vec![];

        // Remaining commits to walk
        let mut rem = VecDeque::new();
        rem.push_front(head);

        while let Some(oid) = rem.pop_front() {
            // Find the commit
            let commit = repository.find_commit(oid).unwrap();

            if let Some(pull) = find_pr_for(&commit, pulls.clone())A {
                unimplemented!();
            }
        }

        unimplemented!();
    }
}

fn find_pr_for(
    commit: &git2::Commit,
    prs: impl Iterator<Item = github::Pull>
) -> Option<github::Pull> {
    unimplemented!();
}
