use crate::config::TagFormat;

use git2;
use semver::Version;

use std::path::Path;

pub struct Repository {
    inner: git2::Repository,
    tags: Vec<String>,
}

impl Repository {
    pub fn open(path: &Path) -> Repository {
        let inner = match git2::Repository::open(path) {
            Ok(v) => v,
            Err(e) => panic!("failed to open: {}", e),
        };

        let tags = inner.tag_names(None).unwrap()
            .iter()
            .filter_map(|name| name.map(|s| s.to_string()))
            .collect();

        Repository {
            inner,
            tags,
        }
    }

    pub fn tags(&self) -> &[String] {
        &self.tags[..]
    }
}

pub fn tag_for(name: &str, version: &Version, format: TagFormat) -> String {
    match format {
        TagFormat::VersionOnly => {
            format!("v{}", version)
        }
        TagFormat::NameVersion => {
            format!("{}-{}", name, version)
        }
    }
}

/*
    let repo = match Repository::open("/Users/carllerche/Code/Tokio/tokio") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    let mut walk = repo.revwalk().unwrap();
    walk.push_range("tokio-0.1.13..origin/master").unwrap();

    for res in walk {
        println!("{:?}", res.unwrap());
    }
 */
