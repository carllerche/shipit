use crate::config::TagFormat;

use git2;
use semver::Version;
use url::{Url, Host};

use std::path::Path;

pub struct Repository {
    /// Git repository handle
    inner: git2::Repository,

    /// Repository tags
    tags: Vec<String>,

    origin_url: Url,
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

        let origin_url = {
            let origin = inner.find_remote("origin").unwrap();
            let url = Url::parse(origin.url().unwrap()).unwrap();

            // Only github origins are supported.
            assert_eq!(url.host(), Some(Host::Domain("github.com")));

            url
        };

        Repository {
            inner,
            tags,
            origin_url,
        }
    }

    pub fn tags(&self) -> &[String] {
        &self.tags[..]
    }

    pub fn origin_url(&self) -> &Url {
        &self.origin_url
    }

    /// TODO: Make branch configurable
    pub fn wut(&self) -> Result<(), Box<::std::error::Error>> {
        let mut walk = self.inner.revwalk().unwrap();
        walk.push_ref("refs/remotes/origin/master");

        println!("~~~ iter");

        // Walk all the commits looking for the first release commit that
        // includes....
        for res in walk {
            let oid = res?;
            /*
            let oid = match res {
                Ok(v) => v,
                Err(_) => unimplemented!(),
            };

            let before = self.inner.find_tree(commit).unwrap();
            let after = self.inner.find_tree(first).unwrap();

            let diff = self.inner.diff_tree_to_tree(Some(&before), Some(&after), None)
                .unwrap();

            diff.foreach(
            */
        }

        println!("~~~ done");
        Ok(())
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
