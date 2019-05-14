use crate::cargo;
use crate::git;
use crate::{Config, Workspace};
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

/// Check a workspace, ensuring it is valid
pub fn run(workspace: &Workspace, config: &Config) {
    // Initialize a new Github client
    // let github = github::Client::new(&config.system);

    // Open the git repository
    let repository = git::Repository::open(workspace.root());

    let remote = git::Ref::Remote {
        remote: "origin".to_string(),
        name: "master".to_string(),
    };

    // All commits
    let mut commits = HashMap::new();

    // Commits that are relevant to a particular project
    let mut per_package = HashMap::new();

    // Iterate over all packages managed by shipit.
    for name in config.project.packages.keys() {
        // Get the workspace package.
        let package = workspace.get_member(name).unwrap();

        // Get list of published versions
        let mut published = cargo::published_versions(name);

        // Sort versions. The latest version is last.
        published.sort();

        // Next, find the last published ref. This is used as the starting point
        // to determine if there are unpublished changes

        if let Some(version) = published.last() {
            // Generate the tag
            let tag_name = config.project.tag_format.format(name, version);
            let tag = git::Ref::Tag(tag_name);

            // First, ensure that the tag is contained by the master branch
            assert!(
                repository.is_descendant_of(&remote, &tag),
                "tag not in history of branch"
            );

            for commit in repository.commits_in_range(&tag, &remote) {
                let changed_files = commits
                    .entry(commit)
                    .or_insert_with(|| repository.files_changed(commit));

                if changed_files.modifies(package, &workspace) {
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
    }

    println!("{:#?}", per_package);

    /*
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
