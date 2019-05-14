use crate::config::{error, TagFormat};
use crate::{Error, Workspace};

use git2;
use std::collections::HashMap;
use std::path::Path;

/// Project specific configuration specified by a `.shipit.yml` file.
#[derive(Debug, Default)]
pub struct Project {
    /// Packages managed by `shipit`.
    pub packages: HashMap<String, Package>,

    /// How to format git tags
    pub tag_format: TagFormat,

    /// Git SHA at which `shipit` begins managing the release process.
    pub initial_commit: Option<git2::Oid>,
}

/// Package configuration
#[derive(Debug)]
pub struct Package {
    /*
/// `Some` when releases are tagged.
pub tag_format: Option<TagFormat>,

/// Path to changelog, `None` if no changelog is maintained
pub changelog: Option<PathBuf>,
*/}

impl Project {
    pub const DEFAULT_FILE_NAME: &'static str = "shipit.toml";

    pub fn load(workspace: &Workspace) -> Result<Project, Error> {
        let file = workspace.root().join(Project::DEFAULT_FILE_NAME);
        let toml = load_file(&file)?;

        let packages = toml
            .packages
            .into_iter()
            .map(|package| (package, Package {}))
            .collect();

        let tag_format: TagFormat = toml.git.tag_format.parse()?;

        let initial_commit = None;

        let project = Project {
            packages,
            tag_format,
            initial_commit,
        };

        project.check(workspace)?;

        Ok(project)
    }

    pub fn check(&self, workspace: &Workspace) -> Result<(), Error> {
        if !self.tag_format.includes_name() {
            if self.packages.len() > 1 {
                return Err(error::InvalidTagFormat.into());
            }
        }

        for package in self.packages.keys() {
            if !workspace.get_member(package).is_some() {
                return Err(error::UnknownPackage::new(package).into());
            }
        }

        Ok(())
    }

    pub fn to_string(&self) -> Result<String, Error> {
        use std::fmt::Write;

        let mut out = String::new();

        writeln!(out, "packages = [")?;

        let mut names: Vec<&str> = self.packages.keys().map(|s| &s[..]).collect();
        names.sort();

        for name in &names {
            writeln!(out, "  {:?},", name)?;
        }

        writeln!(out, "]")?;
        writeln!(out, "")?;

        writeln!(out, "[git]")?;
        writeln!(out, "tag-format = \"{}\"", self.tag_format)?;

        if let Some(ref initial_commit) = self.initial_commit {
            writeln!(out, "initial-commit = \"{}\"", initial_commit)?;
        }

        Ok(out)
    }
}

impl Package {
    /*
    fn load(name: &str, toml: &toml::Project) -> Package {
        let package_toml = toml.packages.get(name);

        // let initial_managed_version = package_toml.and_then(|p| p.managed_version.clone());

        // Get the workspace global tag format
        let mut tag_format =
            toml.git
                .as_ref()
                .and_then(|git| match git.tag_format.as_ref().map(|s| &s[..]) {
                    Some(VERSION_ONLY) => Some(TagFormat::VersionOnly),
                    Some(NAME_VERSION) => Some(TagFormat::NameVersion),
                    Some(_) => panic!(),
                    None => None,
                });

        // Check package specific tag configuration
        if let Some(false) = package_toml.and_then(|p| p.tag) {
            tag_format = None;
        }

        let initial_commit = package_toml
            .and_then(|p| p.managed_sha.as_ref())
            .map(|sha| sha.parse().unwrap());

        Package {
            initial_commit,
            tag_format,
            changelog: None,
        }
    }
    */
}

pub fn load_file(path: &Path) -> Result<toml::Project, Error> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::open(path)?;

    let mut dst = vec![];
    file.read_to_end(&mut dst)?;

    let config = toml::from_slice(&dst)?;
    Ok(config)
}

mod toml {
    use serde_derive::Deserialize;
    pub use toml::{de, from_slice};

    /// Ship it TOML configuration representation
    #[derive(Debug, Deserialize)]
    pub struct Project {
        /// Global git configuration values
        pub git: Git,

        /*
        /// Global changelog configuration values
        pub changelog: Option<Changelog>,
        */
        /// Package specific configuration
        pub packages: Vec<String>,
    }

    /// Global git configuration values
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct Git {
        /// How to format git tags
        pub tag_format: String,

        /// First commit from which `shipit` is managing the project.
        pub initial_commit: Option<String>,
    }

    /*
    /// Global changelog configuration values
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct Changelog {}

    /// Package specific configuration values
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct Package {
        /// The first release version that is managed by shipit.
        pub managed_version: Option<Version>,

        /// The git sha at which `shipit` begins managing the release process.
        pub managed_sha: Option<String>,

        /// Whether or not the package should be taged on release.
        pub tag: Option<bool>,

        /// `true` when a changelog is maintained for the package.
        pub changelog: Option<bool>,
    }
    */
}
