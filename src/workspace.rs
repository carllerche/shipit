use crate::manifest;
use crate::package::Package;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Workspace {
    members: HashMap<String, Package>,

    root: PathBuf,
}

impl Workspace {
    pub fn get_member(&self, name: &str) -> Option<&Package> {
        self.members.get(name)
    }

    pub fn members(&self) -> impl ExactSizeIterator<Item = &Package> {
        self.members.values()
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Load a workspace from manifest files
    pub fn load(root: &Path) -> Workspace {
        let root = root.canonicalize().unwrap();

        let manifest = manifest::Manifest::load(&root);

        let mut workspace = Workspace {
            members: HashMap::new(),
            root: root.clone(),
        };

        if let Some(manifest_workspace) = manifest.workspace {
            let members = match manifest_workspace.members {
                Some(members) => members,
                None => panic!("workspace must specify members"),
            };

            for member in members {
                let member_path = root.join(&member);
                let manifest = manifest::Manifest::load(&root.join(&member));

                let package = match manifest.package {
                    Some(package) => package,
                    None => panic!("workspace members must specify a package"),
                };

                let name = package.name.as_ref().unwrap();

                // Expect unique package names in the workspace.
                assert!(
                    !workspace.members().any(|package| package.name() == name),
                    "duplicate package names"
                );

                let package = Package::new(package, &member_path.strip_prefix(&root).unwrap());

                workspace
                    .members
                    .insert(package.name().to_string(), package);
            }
        }

        // Expect the root package exists in the workspace
        if let Some(package) = manifest.package {
            let name = package.name.unwrap();

            assert!(
                workspace.members().any(|package| package.name() == name),
                "root package not listed in workspace members"
            );
        }

        workspace
    }
}
