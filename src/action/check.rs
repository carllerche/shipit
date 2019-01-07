use crate::{Workspace, Config};

/// Check a workspace, ensuring it is valid
pub fn check(workspace: &Workspace, config: &Config) {
    for member in workspace.members() {
        let config = &config.packages[member.name()];

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
