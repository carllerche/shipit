use semver::Version;
use serde_derive::Deserialize;
use toml;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// Represents the needed `Cargo.toml` data.
#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub package: Option<Package>,
    pub workspace: Option<Workspace>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Package {
    pub name: Option<String>,
    pub version: Option<Version>,
}

#[derive(Debug, Deserialize)]
pub struct Workspace {
    pub members: Option<Vec<String>>,
}

impl Manifest {
    pub fn load(root: &Path) -> Manifest {
        let mut file = File::open(&root.join("Cargo.toml")).unwrap();

        let mut dst = vec![];
        file.read_to_end(&mut dst).unwrap();

        let manifest: Manifest = toml::from_slice(&dst).unwrap();
        manifest
    }
}
