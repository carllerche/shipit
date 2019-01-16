use crate::changelog;
use crate::Workspace;

use semver::Version;
use toml;

use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

/// Configures `shipit` behavior.
#[derive(Debug, Default)]
pub struct Config {
    pub project: Project,
}

/// Project specific configuration specified by a `.shipit.yml` file.
#[derive(Debug, Default)]
pub struct Project {
    pub packages: HashMap<String, Package>,
}

/// Package configuration
#[derive(Debug)]
pub struct Package {
    /// Crate version at which `shipit` begins managing to the release process.
    ///
    /// Any version prior to this will be ignored.
    pub initial_managed_version: Option<Version>,

    /// `Some` when releases are tagged.
    pub tag_format: Option<TagFormat>,

    /// Path to changelog, `None` if no changelog is maintained
    pub changelog: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TagFormat {
    /// Example: `v0.1.1`
    VersionOnly,

    /// Example: `tokio-0.1.1`
    NameVersion,
}

/// String representation of `TagFormat::VersionOnly`.
const VERSION_ONLY: &str = "version";

/// String representation of `TagFormat::NameVersion`.
const NAME_VERSION: &str = "name-version";

#[derive(Debug)]
pub enum LoadError {
    NotFound,
}

pub const DEFAULT_FILE_NAME: &str = ".shipit.toml";

impl Project {
    pub fn load(workspace: &Workspace) -> Result<Project, LoadError> {
        let file = workspace.root().join(DEFAULT_FILE_NAME);
        let toml = load_file(&file)?;

        let mut project = Project {
            packages: HashMap::new(),
        };

        for member in workspace.members() {
            project.packages.insert(
                member.name().to_string(),
                Package::load(member.name(), &toml));
        }

        Ok(project)
    }

    pub fn write(&self, path: &Path) -> Result<(), Box<::std::error::Error>> {
        use std::fmt::Write;
        use std::io::Write as IoWrite;

        // All changelog fields must be the default value
        assert!(self.packages.values().all(|package| {
            // Fun
            package.changelog.as_ref().map(|p| p.as_ref()) ==
                Some(Path::new(changelog::DEFAULT_FILE_NAME))
        }), "unimplemented: customized changelog configuration");

        // Ensure all tag_format values are the same
        let mut packages = self.packages.values();
        let tag_format = packages.next().unwrap().tag_format;

        assert!(packages.all(|package| {
            package.tag_format == tag_format
        }), "unimplemented: different tag_format configuration values");

        let mut out =
            "# Automated CHANGELOG management\n\
             [changelog]\n\n".to_string();

        if let Some(tag_format) = tag_format {
            let tag_format_str = match tag_format {
                TagFormat::VersionOnly => VERSION_ONLY,
                TagFormat::NameVersion => NAME_VERSION,
            };

            writeln!(out, "[git]")?;
            writeln!(out, "tag-format = {:?}", tag_format_str)?;
        }

        let mut names: Vec<&str> = self.packages.keys()
            .map(|s| &s[..])
            .collect();

        names.sort();

        for name in &names {
            let package = &self.packages[*name];

            if let Some(ref initial_version) = package.initial_managed_version {
                writeln!(out, "")?;
                writeln!(out, "[packages.{}]", name)?;
                writeln!(out, "managed-version = \"{}\"", initial_version)?;
            }
        }

        let mut file = File::create(path.join(DEFAULT_FILE_NAME))?;
        file.write_all(out.as_bytes())?;

        Ok(())
    }
}

impl Package {
    fn load(name: &str, toml: &repr::Project) -> Package {
        let package_toml = toml.packages.get(name);

        let initial_managed_version =
            package_toml.and_then(|p| p.managed_version.clone());

        // Get the workspace global tag format
        let mut tag_format = toml.git.as_ref()
            .and_then(|git| {
                match git.tag_format.as_ref().map(|s| &s[..]) {
                    Some(VERSION_ONLY) => unimplemented!(),
                    Some(NAME_VERSION) => Some(TagFormat::NameVersion),
                    Some(_) => panic!(),
                    None => None,
                }
            });

        // Check package specific tag configuration
        if let Some(false) = package_toml.and_then(|p| p.tag) {
            tag_format = None;
        }

        Package {
            initial_managed_version,
            tag_format,
            changelog: None,
        }
    }
}

impl TagFormat {
    pub fn all() -> &'static [TagFormat] {
        use self::TagFormat::*;

        &[NameVersion, VersionOnly]
    }
}

impl LoadError {
    pub fn is_not_found(&self) -> bool {
        match *self {
            LoadError::NotFound => true,
        }
    }
}

impl From<io::Error> for LoadError {
    fn from(src: io::Error) -> LoadError {
        use std::io::ErrorKind::*;

        match src.kind() {
            NotFound => LoadError::NotFound,
            _ => unimplemented!(),
        }
    }
}

impl From<toml::de::Error> for LoadError {
    fn from(src: toml::de::Error) -> LoadError {
        unimplemented!("error = {:?}", src);
    }
}

pub fn load_file(path: &Path) -> Result<repr::Project, LoadError> {
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::open(path)?;

    let mut dst = vec![];
    file.read_to_end(&mut dst)?;

    let config = toml::from_slice(&dst)?;
    Ok(config)
}

mod repr {
    use semver::Version;
    use serde_derive::Deserialize;

    use std::collections::HashMap;

    /// Ship it TOML configuration representation
    #[derive(Debug, Deserialize)]
    pub struct Project {
        /// Global git configuration values
        pub git: Option<Git>,

        /// Global changelog configuration values
        pub changelog: Option<Changelog>,

        /// Package specific configuration
        pub packages: HashMap<String, Package>,
    }

    /// Global git configuration values
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct Git {
        /// How to format git tags
        pub tag_format: Option<String>,
    }

    /// Global changelog configuration values
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct Changelog {
    }

    /// Package specific configuration values
    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct Package {
        /// The first release version that is managed by shipit.
        pub managed_version: Option<Version>,

        /// Whether or not the package should be taged on release.
        pub tag: Option<bool>,

        /// How to format package git tags.
        pub tag_format: Option<String>,

        /// `true` when a changelog is maintained for the package.
        pub changelog: Option<bool>,
    }
}
