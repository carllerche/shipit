use crate::cargo;
use crate::changelog;
use crate::config;
use crate::git;
use crate::Workspace;

use semver::Version;

pub fn run(workspace: &Workspace, config: Option<&config::Project>) {
    use std::fs::File;
    use std::io::prelude::*;

    if config.is_some() {
        let path = workspace.root().join(config::Project::DEFAULT_FILE_NAME);
        panic!("{} already exists", path.display());
    }

    // Open the git repository
    let repository = git::Repository::open(workspace.root());
    let sha = repository.deref(&git::Ref::head("master")).unwrap();

    let mut config = config::Project::default();

    // Start by loading the initial version
    for member in workspace.members() {
        config.packages.insert(
            member.name().to_string(),
            config::Package {
                initial_managed_version: Some(member.manifest_version().clone()),
                initial_managed_sha: None,
                tag_format: None,
                changelog: Some(changelog::DEFAULT_FILE_NAME.into()),
            },
        );
    }

    let zero_one_zero = Version {
        major: 0,
        minor: 1,
        patch: 0,
        pre: vec![],
        build: vec![],
    };

    if workspace.members().len() == 1 {
        let member = &workspace.members().next().unwrap();

        let mut published = cargo::published_versions(member.name());
        published.sort();

        if repository.tags().is_empty() {
            // Load published crates
            let patch_only = published.iter().all(|version| *version < zero_one_zero);

            if patch_only {
                // Set to VersionOnly
                config.packages.get_mut(member.name()).unwrap().tag_format =
                    Some(config::TagFormat::VersionOnly);
            } else {
                // Otherwise, do not tag the crate
                config.packages.get_mut(member.name()).unwrap().initial_managed_sha =
                    Some(sha);
            }
        } else {
            let last_release = match published.last() {
                Some(version) => version,
                None => unimplemented!("tags but no releases"),
            };

            let format = config::TagFormat::all()
                .into_iter()
                .find(|format| {
                    let tag = git::tag_for(member.name(), last_release, **format);

                    repository.tags().contains(&tag)
                })
                .map(|format| *format);

            if format.is_none() {
                panic!("could not match tag format");
            }

            config.packages.get_mut(member.name()).unwrap().tag_format = format;
        }
    } else {
        // The tag format must be `{name}-{version}` unless tagging is skipped
        for member in workspace.members() {
            let mut published = cargo::published_versions(member.name());
            published.retain(|v| *v >= zero_one_zero);
            published.sort();

            let package = config.packages.get_mut(member.name()).unwrap();

            let last_release = match published.last() {
                Some(version) => version,
                _ => {
                    package.tag_format = Some(config::TagFormat::NameVersion);
                    continue;
                }
            };

            // Try to get the tag
            let tag = git::tag_for(
                member.name(),
                last_release,
                config::TagFormat::NameVersion);

            if repository.tags().contains(&tag) {
                package.tag_format = Some(config::TagFormat::NameVersion);
            } else {
                package.tag_format = None;
                package.initial_managed_sha = Some(sha);
            }
        }

        let all_have_initial_sha =
            workspace.members().all(|member| {
                config.packages.get(member.name()).unwrap()
                    .initial_managed_sha.is_some()
            });

        if !repository.tags().is_empty() && all_have_initial_sha {
            panic!("repository uses unsupported tag format");
        }
    }

    let out = config.to_string().unwrap();
    let path = workspace.root().join(config::Project::DEFAULT_FILE_NAME);

    let mut file = File::create(&path).unwrap();
    file.write_all(out.as_bytes()).unwrap();
}
