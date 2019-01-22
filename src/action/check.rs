use crate::cargo;
use crate::config;
use crate::git;
use crate::Workspace;

/// Check a workspace, ensuring it is valid
pub fn run(workspace: &Workspace, config: &config::Project) {
    let repository = git::Repository::open(workspace.root());

    for member in workspace.members() {
        let config = &config.packages[member.name()];

        let mut published = cargo::published_versions(member.name());

        if let Some(init_version) = config.initial_managed_version.as_ref() {
            published.retain(|version| version >= init_version);
        }

        if let Some(tag_format) = config.tag_format {
            for version in &published {
                let tag = git::tag_for(member.name(), version, tag_format);

                if !repository.tags().contains(&tag) {
                    panic!("missing tag `{}`", tag);
                }
            }
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
