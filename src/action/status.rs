use crate::cargo;
use crate::github;
use crate::{Config, Workspace};

use slog::*;

/// Check a workspace, ensuring it is valid
pub fn run(workspace: &Workspace, config: &Config) {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let log = slog::Logger::root(drain, o!());
    let github = github::Client::new(&config.system);
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

    for (name, package_config) in &config.project.packages {
        let package = workspace.get_member(name).unwrap();

        let mut published = cargo::published_versions(name);

        if let Some(init_version) = package_config.initial_managed_version.as_ref() {
            published.retain(|version| version >= init_version);
        }

        // Sort versions. The latest version is last.
        published.sort();

        if let Some(_tag_format) = package_config.tag_format {
            // for version in &published {
            //     let tag = git::tag_for(member.name(), version, tag_format);
            //     if !repository.tags().contains(&tag) && *version >= zero_one_zero {
            //         panic!("missing tag `{}`", tag);
            //     }
            // }
            info!(log, "TODO: identify missing tags here")
        } else {
            warn!(log, "NO TAGGING = {}", name);
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
