use crate::{Workspace, Config};
use crate::cargo;
use crate::git;
use crate::github;
use crate::package;

use semver::Version;

/// Check a workspace, ensuring it is valid
pub fn run(workspace: &Workspace, config: &Config) {
    let github = github::Client::new();
    github.prs();

    if true { panic!() }

    let repository = git::Repository::open(workspace.root());

    let zero_one_zero = Version {
       major: 0,
       minor: 1,
       patch: 0,
       pre: vec![],
       build: vec![],
    };

    for member in workspace.members() {
        let config = &config.packages[member.name()];

        let mut published = cargo::published_versions(member.name());

        if let Some(init_version) = config.initial_managed_version.as_ref() {
            published.retain(|version| version >= init_version);
        }

        // Sort versions. The latest version is last.
        published.sort();

        if let Some(tag_format) = config.tag_format {
            for version in &published {
                let tag = git::tag_for(member.name(), version, tag_format);

                if !repository.tags().contains(&tag) && *version >= zero_one_zero {
                    panic!("missing tag `{}`", tag);
                }
            }
        } else {
            println!("NO TAGGING = {}", member.name());
            // repository.wut();
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
}
