use crate::{cargo, git, manifest};

use semver::Version;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Package {
    /// Package name
    name: String,

    /// Version listed in the manifest file
    manifest_version: Version,

    /// Versions published to crates.io
    published_versions: Vec<Version>,

    /// Path on disk
    path: PathBuf,
}

impl Package {
    pub fn new(manifest: manifest::Package, path: &Path) -> Package {
        // Get the necessary manifest data
        let name = manifest.name.unwrap();
        let manifest_version = manifest.version.unwrap();

        // Fetch all versions published to crates.io
        let published_versions = cargo::published_versions(&name);

        Package {
            name,
            manifest_version,
            published_versions,
            path: path.canonicalize().unwrap(),
        }
    }

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
        if self.published_versions.is_empty() {
            // TODO: Ensure tag missing
            return;
        }

        let tag = format!("{}-{}", self.name, self.manifest_version);

        assert!(repository.tags().contains(&tag),
                "tag = {}; published = {:?}",
                tag, self.published_versions);
    }
}
