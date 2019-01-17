use crate::cargo;
use crate::git;
use crate::github;
use crate::package;
use crate::{Config, Workspace};

use semver::Version;

use slog::*;

/// Check a workspace, ensuring it is valid
pub fn run(workspace: &Workspace, config: &Config) {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let log = slog::Logger::root(drain, o!());
    let github = github::Client::new();
    let pulls = github.prs();

    for pr in pulls.iter() {
        info!(log, "{:?}\n", pr);
    }

    let repository = git::Repository::open(workspace.root());

    let zero_one_zero = Version {
        major: 0,
        minor: 1,
        patch: 0,
        pre: vec![],
        build: vec![],
    };

    let mut missing_tags: Vec<String> = vec![];
    for member in workspace.members() {
        // * Get initial supported version.
        // * Get list of published crates after that
        // * Ensure tags for each
        // * If changelog, check format

        let config = &config.packages[member.name()];

        let mut published = cargo::published_versions(member.name());

        if let Some(init_version) = config.initial_managed_version.as_ref() {
            published.retain(|version| version >= init_version);
        }

        // Sort versions. The latest version is last.
        published.sort();
        if let Some(tag_format) = config.tag_format {
            for version in &published {
                let tag = git::tag_for(member.name(), &version, tag_format);
                if !repository.tags().contains(&tag) && *version >= zero_one_zero {
                    missing_tags.push(tag)
                }
            }
        } else {
            warn!(log, "NO TAGGING = {}", member.name());

            // TODO: walk commits and find in Git
            // repository.wut();
        }
    }

    if !&missing_tags.is_empty() {
        warn!(
            log,
            "The following missing tag(s) were identified: {:?}", missing_tags
        );
    } else {
        info!(log, "All tags are Ok!");

        // TODO: implement has_changelog to Workspace
        /*
        if member.has_changelog() {
            member.unpublished(&repository);
        }
        */
    }
}
