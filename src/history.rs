use crate::git::{self, Ref, Repository};
use crate::github;
use crate::util;
use git2;

use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug)]
pub struct History {
    commits: Vec<Commit>,
}

#[derive(Debug)]
pub struct Commit {
    /// The commit object ID
    pub oid: git2::Oid,

    /// The pull requests that the commit originated from.
    ///
    /// There can possibly be multiple commits if pull requests are merged into
    /// other pull requests.
    pub pulls: Vec<github::PullRequest>,
}

impl History {
    pub fn commits(&self) -> &[Commit] {
        &self.commits[..]
    }

    /// Load git history for a branch, including pull requests.
    pub fn load<T>(
        // The repository to load from
        repository: &mut Repository,

        // The git branch to traverse
        head: &git::Ref,

        // Refs to traverse until
        terminals: &[git::Ref],

        // Handle to the github client
        github: &github::Client<T>,
    ) -> History
    where
        T: github::Transport,
    {
        for terminal in terminals {
            assert!(
                repository.is_descendant_of(head, terminal),
                "head = {:?}; terminal = {:?}",
                head,
                terminal);
        }

        println!("TERMINALS = {:#?}", terminals);

        let mut terminals = terminals.to_vec();

        // Find the oldest push date. This is used to limit the number of pull
        // requests being checked.
        let pushed_date = if !terminals.is_empty() {
            Some(github.pushed_date(&terminals).unwrap())
        } else {
            None
        };

        // An iterator to pull requests
        let mut pulls = util::Replay::new({
            github.pull_requests()
                .take_while(|pull| {
                    if let Some(date) = pushed_date {
                        match pull.as_ref() {
                            Ok(pull) if pull.updated_at < date => false,
                            _ => true,
                        }
                    } else {
                        true
                    }
                })
                // The error must be clone
                .map(|res| res.map_err(Rc::new))
        });

        // Commits part of the history
        let mut commits = vec![];

        // Remaining commits to walk
        let mut rem = VecDeque::new();
        rem.push_back(head.clone());

        while let Some(oid) = rem.pop_front() {
            // Find the commit
            let git_commit = repository.find_commit(&oid).unwrap();

            if git_commit.parent_count() > 1 {
                unimplemented!();
            } else {
                let pull = pulls
                    .iter()
                    .find(|pull| {
                        git_commit.id() == pull.as_ref().unwrap().merge_commit
                    });

                if let Some(pull) = pull {
                    let pull = pull.unwrap();

                    commits.push(Commit {
                        oid: git_commit.id(),
                        pulls: vec![pull],
                    });
                }

                let mut reached_terminal = false;

                terminals.retain(|terminal| {
                    let matches = repository.references(terminal, git_commit.id());
                    reached_terminal |= matches;
                    !matches
                });

                let parent = git_commit.parent(0).unwrap();

                if reached_terminal {
                    // Check if the parent is descendent of any remaining
                    // termianls
                    let more_terminals = terminals.iter().any(|terminal| {
                        repository.is_descendant_of(&Ref::Sha(parent.id()), terminal)
                    });

                    if !more_terminals {
                        continue;
                    }
                }

                rem.push_back(Ref::Sha(parent.id()));
            }
        }

        History {
            commits,
        }
    }
}
