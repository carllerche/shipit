use semver::Version;
use std::collections::HashMap;

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
    initial_managed_version: Option<Version>,

    /// `Some` when releases are tagged.
    tag: Option<TagFormat>,

    /// True when a changelog is maintained
    changelog: bool,
}

#[derive(Debug)]
pub enum TagFormat {
    /// Example: `v0.1.1`
    VersionOnly,

    /// Example: `tokio-0.1.1`
    NameVersion,
}

impl Default for Package {
    fn default() -> Package {
        unimplemented!();
    }
}
