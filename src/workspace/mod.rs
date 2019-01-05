mod manifest;

use crate::git;

use semver::Version;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Workspace {
    members: Vec<Package>,
}

#[derive(Debug)]
pub struct Package {
    /// Package name
    name: String,

    /// Version listed in the manifest file
    manifest_version: Version,

    /// Path on disk
    path: PathBuf,
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

                let name = package.name.unwrap();

                // Expect unique package names in the workspace.
                assert!(!workspace.members.iter().any(|package| {
                    package.name == name
                }), "duplicate package names");

                workspace.members.push(Package {
                    name,
                    manifest_version: package.version.unwrap(),
                    path: path.canonicalize().unwrap(),
                });
            }
        }

        // Expect the root package exists in the workspace
        if let Some(package) = manifest.package {
            let name = package.name.unwrap();

            assert!(workspace.members.iter().any(|package| {
                package.name == name
            }), "root package not listed in workspace members");
        }

        workspace
    }

    pub fn members(&self) -> &[Package] {
        &self.members[..]
    }
}

impl Package {
    /// Return the package name
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn manifest_version(&self) -> &Version {
        &self.manifest_version
    }

    pub fn has_changelog(&self) -> bool {
        self.path.join("CHANGELOG.md").exists()
    }

    pub fn unpublished(&self, repository: &git::Repository) {
        let tag = format!("{}-{}", self.name, self.manifest_version);

        assert!(repository.tags().contains(&tag), "tag = {}", tag);
    }
}
