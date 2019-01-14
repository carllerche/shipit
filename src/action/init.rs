use crate::cargo;
use crate::changelog;
use crate::config::{self, Config, Package, TagFormat};
use crate::git;
use crate::Workspace;

use semver::Version;

pub fn run(workspace: &Workspace, config: Option<&Config>) {
    if config.is_some() {
        let path = workspace.root().join(config::DEFAULT_FILE_NAME);
        panic!("{} already exists", path.display());
    }

    // Open the git repository
    let repository = git::Repository::open(workspace.root());

    let mut config = Config::default();

    // Start by loading the initial version
    for member in workspace.members() {
        config.packages.insert(member.name().to_string(), Package {
            initial_managed_version: Some(member.manifest_version().clone()),
            tag_format: None,
            changelog: Some(changelog::DEFAULT_FILE_NAME.into()),
        });
    }

    if workspace.members().len() == 1 {
        let member = &workspace.members()[0];

        let mut published = cargo::published_versions(member.name());
        published.sort();

        let zero_one_zero = Version {
           major: 0,
           minor: 1,
           patch: 0,
           pre: vec![],
           build: vec![],
        };

        if repository.tags().is_empty() {
            // Load published crates
            let patch_only = published
                .iter()
                .all(|version| *version < zero_one_zero);

            if patch_only {
                // Set to VersionOnly
                config.packages.get_mut(member.name()).unwrap()
                    .tag_format = Some(TagFormat::VersionOnly);
            } else {
                // Otherwise, do not tag the crate
            }
        } else {
            let last_release = match published.last() {
                Some(version) => version,
                None => unimplemented!("tags but no releases"),
            };

            let format = TagFormat::all().into_iter()
                .find(|format| {
                    let tag = git::tag_for(
                        member.name(),
                        last_release,
                        **format);

                    repository.tags().contains(&tag)
                })
                .map(|format| *format);

            if format.is_none() {
                panic!("could not match tag format");
            }

            config.packages.get_mut(member.name()).unwrap()
                .tag_format = format;
        }
    } else {
        // The tag format must be `{name}-{version}`
        for member in workspace.members() {
            config.packages.get_mut(member.name()).unwrap()
                .tag_format = Some(TagFormat::NameVersion);
        }
    }

    config.write(workspace.root());
}
