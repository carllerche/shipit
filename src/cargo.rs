use crate::workspace::Package;

use semver::Version;
use serde_derive::Deserialize;

/// Returns the most recent version published to crates.io
pub fn is_published(package: &Package) -> bool {
    // Compute the URL
    let url = crates_index_url(package.name());

    let body = reqwest::get(&url).unwrap()
        .text().unwrap();

    for line in body.lines() {
        let published: PublishedPackage = serde_json::from_str(line).unwrap();

        if published.vers == *package.manifest_version() {
            return true;
        }
    }

    false
}

fn crates_index_url(name: &str) -> String {
    format!(
        "https://raw.githubusercontent.com/rust-lang/crates.io-index/master/{}/{}/{}",
        &name[0..2],
        &name[2..4],
        name)
}

#[derive(Debug, Deserialize)]
struct PublishedPackage {
    name: String,
    vers: Version,
}
