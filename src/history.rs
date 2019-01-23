use crate::git::{self, Ref, Repository};
use crate::github;
use crate::util;
use git2;

use std::collections::VecDeque;
use std::rc::Rc;

pub struct History {
    commits: Vec<()>,
}

pub struct Commit {
    /// The commit object ID
    oid: git2::Oid,

    /// The pull requests that the commit originated from.
    ///
    /// There can possibly be multiple commits if pull requests are merged into
    /// other pull requests.
    pulls: Vec<github::PullRequest>,
}

impl History {
    /// Load git history for a branch, including pull requests.
    pub fn load(
        // The repository to load from
        repository: &mut Repository,

        // The git branch to traverse
        head: &git::Ref,

        // Refs to traverse until
        terminals: &[git::Ref],

        // Handle to the github client
        github: &github::Client,
    ) -> History {
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
        let pushed_date = github.pushed_date(&terminals).unwrap();

        // An iterator to pull requests
        let mut pulls = util::Replay::new({
            github.pull_requests()
                .take_while(|pull| {
                    match pull.as_ref() {
                        Ok(pull) if pull.updated_at < pushed_date => false,
                        _ => true,
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
            let commit = repository.find_commit(&oid).unwrap();

            if commit.parent_count() > 1 {
                unimplemented!();
            } else {
                let pull = pulls
                    .iter()
                    .find(|pull| commit.id() == pull.as_ref().unwrap().merge_commit);

                if let Some(pull) = pull {
                    let pull = pull.unwrap();
                    println!("PR MATCH; {:?} -- (#{})", commit.summary(), pull.number);
                }

                let mut reached_terminal = false;

                terminals.retain(|terminal| {
                    let matches = repository.references(terminal, commit.id());
                    reached_terminal |= matches;
                    !matches
                });

                let parent = commit.parent(0).unwrap();

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
