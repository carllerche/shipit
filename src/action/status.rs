use crate::cargo;
use crate::git;
use crate::github;
use crate::{Config, Workspace};

use slog::*;
use git2;

use std::collections::HashMap;

/*
/// Unpublished changes
struct Unpublished {
    /// Pull requests that are partially or fully unreleased.
    pulls: HashMap<u64, github::Pull>,

    /// Map of package name to pull request IDs.
    unpublished: HashMap<String, Vec<u64>>,
}
*/

struct Commit {
    sha: git2::Oid,
    modified: git::FileSet,
}

/// Check a workspace, ensuring it is valid
pub fn run(workspace: &Workspace, config: &Config) {
    /*
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let log = slog::Logger::root(drain, o!());
    let github = github::Client::new(&config.system);

    let pulls = github.prs();

    for pr in pulls.iter() {
        info!(log, "{:?}\n", pr);
    }
    */

    let repository = git::Repository::open(workspace.root());

    /*
    let zero_one_zero = Version {
        major: 0,
        minor: 1,
        patch: 0,
        pre: vec![],
        build: vec![],
    };

    // To check:
    //
    // * All tags are contained by the `master` branch
    //
    // Action:
    //
    // * Find the oldest commit in the branch. All searched PRs will need to
    //   have been merged as a decendent of these commits.
    */

    // Collect refs for previous releases
    let mut last_release_refs = vec![];

    // Iterate over all packages managed by shipit.
    for (name, package_config) in &config.project.packages {
        // Get the workspace package.
        let package = workspace.get_member(name).unwrap();

        // Get list of published versions
        let mut published = cargo::published_versions(name);

        // Filter out all versions not managed by shipit
        if let Some(init_version) = package_config.initial_managed_version.as_ref() {
            published.retain(|version| version >= init_version);
        }

        // Sort versions. The latest version is last.
        published.sort();

        // Next, find the last published ref. This is used as the starting point
        // to determine if there are unpublished changes
        if let Some(tag_format) = package_config.tag_format {
            // TODO: Validate that tags exist for all published versions'

            if let Some(version) = published.last() {
                // Generate the tag
                let tag_name = git::tag_for(name, version, tag_format);
                let tag = git::Ref::Tag(tag_name);

                let remote = git::Ref::Remote {
                    remote: "origin",
                    name: "master",
                };

                // First, ensure that the tag is contained by the master branch
                assert!(
                    repository.is_descendant_of(&remote, &tag),
                    "tag not in history of branch"
                );

                last_release_refs.push(tag);

                /*
                for commit in repository.commits_in_range(&tag, &remote) {
                    let changed_files = commits
                        .entry(commit)
                        .or_insert_with(|| repository.files_changed(commit));

                    println!("    + commit = {}", commit);

                    if changed_files.modifies(package, &workspace) {
                        println!("      + modifies package");

                        per_package
                            .entry(package.name())
                            .or_insert(vec![])
                            .push(commit);
                    }
                }
                */
            } else {
                // This is the initial commit
                unimplemented!();
            }
        } else {
            unimplemented!();
        }

        // * Get initial supported version.
        // * Get list of published crates after that
        // * Ensure tags for each
        // * If changelog, check format

        /*
        if member.has_changelog() {
            member.unpublished(&repository);
        }
        */
    }


    /*
    // All commits
    let mut commits = HashMap::new();

    // Commits that are relevant to a particular project
    let mut per_package = HashMap::new();
    */

    /*
    // Iterate over all packages managed by shipit.
    for (name, package_config) in &config.project.packages {
        // Get the workspace package.
        let package = workspace.get_member(name).unwrap();

        // Get list of published versions
        let mut published = cargo::published_versions(name);

        // Filter out all versions not managed by shipit
        if let Some(init_version) = package_config.initial_managed_version.as_ref() {
            published.retain(|version| version >= init_version);
        }

        // Sort versions. The latest version is last.
        published.sort();

        // Next, find the last published ref. This is used as the starting point
        // to determine if there are unpublished changes
        if let Some(tag_format) = package_config.tag_format {
            // TODO: Validate that tags exist for all published versions'

            if let Some(version) = published.last() {
                // Generate the tag
                let tag_name = git::tag_for(name, version, tag_format);
                let tag = git::Ref::Tag(&tag_name);

                let remote = git::Ref::Remote {
                    remote: "origin",
                    name: "master",
                };

                // First, ensure that the tag is contained by the master branch
                assert!(
                    repository.is_descendant_of(&remote, &tag),
                    "tag not in history of branch"
                );

                for commit in repository.commits_in_range(&tag, &remote) {
                    let changed_files = commits
                        .entry(commit)
                        .or_insert_with(|| repository.files_changed(commit));

                    println!("    + commit = {}", commit);

                    if changed_files.modifies(package, &workspace) {
                        println!("      + modifies package");

                        per_package
                            .entry(package.name())
                            .or_insert(vec![])
                            .push(commit);
                    }
                }
            } else {
                // This is the initial commit
                unimplemented!();
            }

            // for version in &published {
            //     let tag = git::tag_for(member.name(), version, tag_format);
            //     if !repository.tags().contains(&tag) && *version >= zero_one_zero {
            //         panic!("missing tag `{}`", tag);
            //     }
            // }
            info!(log, "{}; TODO: identify missing tags here", name)
        } else {
            warn!(log, "NO TAGGING = {}", name);
            unimplemented!();
        }

        // * Get initial supported version.
        // * Get list of published crates after that
        // * Ensure tags for each
        // * If changelog, check format

        /*
        if member.has_changelog() {
            member.unpublished(&repository);
        }
        */
    }

    println!("~~~~~~~~~ ZOMG PRS ~~~~~~~~~~");

    let prs: Vec<_> = github
        .merged_pull_requests()
        .take(30)
        .filter(|pr| pr.merged_at.is_some())
        .take_while(|pr| {
            let sha = pr.merge_commit_sha.clone().unwrap();
            let id = git2::Oid::from_str(&sha).unwrap();

            let is_descendant_of_master = repository.is_descendant_of(
                &git::Ref::Remote {
                    remote: "origin",
                    name: "master",
                },
                &git::Ref::Sha(id),
            );

            println!(
                "contains_key = {:?}; is_descendant_of_master = {:?}; {} (#{})",
                commits.contains_key(&id),
                is_descendant_of_master,
                pr.title,
                pr.number
            );

            commits.contains_key(&id) || !is_descendant_of_master
        })
        .collect();

    for pr in &prs {
        println!(" + {}", pr.title);
    }

    println!("~~~~~~~~~ RESULT ~~~~~~~~~~");

    for (name, commits) in &per_package {
        println!(" + {}", name);

        for commit in commits {
            let pr = prs.iter().find(|pr| {
                let sha = pr.merge_commit_sha.clone().unwrap();
                let id = git2::Oid::from_str(&sha).unwrap();

                id == *commit
            });

            if let Some(pr) = pr {
                println!("    * {}", pr.title);
            }
        }
    }
    */
}
