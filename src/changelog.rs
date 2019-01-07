//! Changelog parser

use std::path::Path;

/// Changelog file
#[derive(Debug)]
pub struct Changelog {
    before: Vec<String>,
    entries: Vec<Entry>,
    after: Vec<String>,
}

#[derive(Debug)]
pub struct Entry {
    header: String,
    contents: Vec<String>,
}

/// Changelog configuration variables
#[derive(Debug)]
pub struct Config {
    /// Markdown header level for release entries in the changelog.
    entry_level: usize,

    /// Header format
    header_format: Vec<String>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            entry_level: 0,
            header_format: vec![
                "{version} - ({Month} {d}, {Y})".to_string()
            ],
        }
    }
}

pub fn load(path: &Path, config: &Config) -> Changelog {
    unimplemented!();
}

impl Changelog {
}
