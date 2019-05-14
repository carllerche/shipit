use crate::cargo;
use crate::config;
use crate::git;
use crate::Workspace;

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
    config.initial_commit = Some(sha);

    // Start by loading the initial version
    for member in workspace.members() {
        config.packages.insert(
            member.name().to_string(),
            config::Package {},
        );
    }

    if workspace.members().len() == 1 {
        detect_tag_format(workspace, &repository, &mut config);
    } else {
        config.tag_format = config::TagFormat::name_version();
    }

    let out = config.to_string().unwrap();
    let path = workspace.root().join(config::Project::DEFAULT_FILE_NAME);

    let mut file = File::create(&path).unwrap();
    file.write_all(out.as_bytes()).unwrap();
}

fn detect_tag_format(workspace: &Workspace, repository: &git::Repository, config: &mut config::Project) {
    let member = &workspace.members().next().unwrap();

    let mut published = cargo::published_versions(member.name());
    published.sort();

    if repository.tags().is_empty() {
        config.tag_format = config::TagFormat::version_only();
    } else {
        let last_release = match published.last() {
            Some(version) => version,
            None => unimplemented!("tags but no releases"),
        };

        let format = config::TagFormat::common()
            .find(|format| {
                let tag = format.format(member.name(), last_release);
                repository.tags().contains(&tag)
            });

        if let Some(format) = format {
            config.tag_format = format;
        } else {
            panic!("could not match tag format");
        }
    }
}
