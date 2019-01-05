mod manifest;

use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Workspace {
    members: Vec<Package>,
}

#[derive(Debug)]
pub struct Package {
    name: String,
    version: String,
    path: PathBuf,
}

impl Workspace {
    pub fn new() -> Workspace {
        Workspace {
            members: vec![],
        }
    }

    pub fn load(root: &Path) -> Workspace {
        let manifest = manifest::Manifest::load(root);

        let mut workspace = Workspace::new();

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
                    version: package.version.unwrap(),
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
}
