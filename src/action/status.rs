use crate::cargo;
use crate::config;
use crate::github;
use crate::Workspace;

use slog::*;

/// Check a workspace, ensuring it is valid
pub fn run(workspace: &Workspace, config: &config::Project) {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let log = slog::Logger::root(drain, o!());
    let github = github::Client::new();
    let pulls = github.prs();

    for pr in pulls.iter() {
        info!(log, "{:?}\n", pr);
    }

    /*
    let repository = git::Repository::open(workspace.root());

    let zero_one_zero = Version {
        major: 0,
        minor: 1,
        patch: 0,
        pre: vec![],
        build: vec![],
    };
    */

    for member in workspace.members() {
        let config = &config.packages[member.name()];

        let mut published = cargo::published_versions(member.name());

        if let Some(init_version) = config.initial_managed_version.as_ref() {
            published.retain(|version| version >= init_version);
        }

        // Sort versions. The latest version is last.
        published.sort();
        if let Some(_tag_format) = config.tag_format {
            // for version in &published {
            //     let tag = git::tag_for(member.name(), version, tag_format);
            //     if !repository.tags().contains(&tag) && *version >= zero_one_zero {
            //         panic!("missing tag `{}`", tag);
            //     }
            // }
            info!(log, "TODO: identify missing tags here")
        } else {
            warn!(log, "NO TAGGING = {}", member.name());
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
