mod manifest;
mod package;

pub use self::package::Package;

use crate::git;

use semver::Version;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Workspace {
    members: Vec<Package>,
}

impl Workspace {
    /// Load a workspace from manifest files
    pub fn load(root: &Path) -> Workspace {
        let manifest = manifest::Manifest::load(root);

        let mut workspace = Workspace {
            members: vec![],
        };

        if let Some(manifest_workspace) = manifest.workspace {
            let members = match manifest_workspace.members {
                Some(members) => members,
                None => panic!("workspace must specify members"),
            };

            for member in members {
                let path = root.join(member);
                let manifest = manifest::Manifest::load(&path);

                let package = match manifest.package {
                    Some(package) => package,
                    None => panic!("workspace members must specify a package"),
                };

                let name = package.name.as_ref().unwrap();

                // Expect unique package names in the workspace.
                assert!(!workspace.members.iter().any(|package| {
                    package.name() == name
                }), "duplicate package names");

                workspace.members.push(Package::new(package, &path));
            }
        }

        // Expect the root package exists in the workspace
        if let Some(package) = manifest.package {
            let name = package.name.unwrap();

            assert!(workspace.members.iter().any(|package| {
                package.name() == name
            }), "root package not listed in workspace members");
        }

        workspace
    }

    pub fn members(&self) -> &[Package] {
        &self.members[..]
    }
}
