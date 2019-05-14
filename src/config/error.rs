use std::error::Error;
use std::fmt;

/*
/// The configuration file is not found
#[derive(Debug)]
pub struct NotFound;

impl fmt::Display for NotFound {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "configuration file not found")
    }
}

impl Error for NotFound {}
*/

/// A package is listed in the shipit configuration but does not exist in the workspace.
#[derive(Debug)]
pub struct UnknownPackage {
    name: String,
}

impl UnknownPackage {
    pub fn new(name: &str) -> UnknownPackage {
        UnknownPackage {
            name: name.to_string(),
        }
    }
}

impl fmt::Display for UnknownPackage {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "unknown package `{}`", self.name)
    }
}

impl Error for UnknownPackage {}

#[derive(Debug)]
pub struct InvalidTagFormat;

impl fmt::Display for InvalidTagFormat {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "invalid tag format")
    }
}

impl Error for InvalidTagFormat {}
