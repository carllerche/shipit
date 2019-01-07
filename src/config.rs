use crate::Workspace;

use semver::Version;

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Configures `shipit` behavior.
#[derive(Debug, Default)]
pub struct Config {
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

#[derive(Debug, Clone, Copy)]
pub enum TagFormat {
    /// Example: `v0.1.1`
    VersionOnly,

    /// Example: `tokio-0.1.1`
    NameVersion,
}

const DEFAULT_CONFIG_FILE: &str = ".shipit.toml";

impl Config {
    pub fn load(workspace: &Workspace) -> Config {
        let file = workspace.root().join(DEFAULT_CONFIG_FILE);
        let toml: toml::Config = toml::Config::load_file(&file);

        let mut config = Config {
            packages: HashMap::new(),
        };

        for member in workspace.members() {
            config.packages.insert(
                member.name().to_string(),
                Package::load(member.name(), &toml));
        }

        config
    }
}

impl Package {
    fn load(name: &str, toml: &toml::Config) -> Package {
        let package_toml = toml.packages.get(name);

        let initial_managed_version =
            package_toml.and_then(|p| p.managed_version.clone());

        let tag_format = toml.git.as_ref()
            .and_then(|git| {
                match git.tag_format.as_ref().map(|s| &s[..]) {
                    Some("version") => unimplemented!(),
                    Some("name-version") => Some(TagFormat::NameVersion),
                    Some(_) => panic!(),
                    None => None,
                }
            });

        Package {
            initial_managed_version,
            tag_format,
            changelog: None,
        }
    }
}

mod toml {
    use semver::Version;
    use serde_derive::Deserialize;

    use std::collections::HashMap;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    #[derive(Debug, Deserialize)]
    pub struct Config {
        pub git: Option<Git>,
        pub packages: HashMap<String, Package>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct Git {
        pub tag_format: Option<String>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub struct Package {
        pub managed_version: Option<Version>,
    }

    impl Config {
        pub fn load_file(path: &Path) -> Config {
            let mut file = File::open(path).unwrap();

            let mut dst = vec![];
            file.read_to_end(&mut dst).unwrap();

            toml::from_slice(&dst).unwrap()
        }
    }
}
