pub use shipit::git::Ref;

use git2::{self, Repository};
use shipit::git as shipit_git;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use tempdir::TempDir;

pub struct Builder {
    dir: TempDir,
    repo: Repository,
    index: git2::Index,
    count: u64,
}

impl Builder {
    /// Create a new git builder
    pub fn new() -> Builder {
        let dir = TempDir::new("shipit-tests").unwrap();
        let repo = Repository::init(dir.path()).unwrap();
        let index = repo.index().unwrap();

        repo.remote_add_fetch("origin", "refs/*:refs/*").unwrap();
        repo.remote_set_url("origin", "https://github.com/example/test").unwrap();

        Builder {
            dir,
            repo,
            index,
            count: 0,
        }
    }

    pub fn repository(&self) -> shipit_git::Repository {
        shipit_git::Repository::open(self.dir.path())
    }

    /// Write a stirng to the path
    pub fn write(&mut self, path: &str, contents: &str) -> &mut Self {
        use std::fs::create_dir_all;

        let abs_path = self.dir.path().join(path);
        let abs_parent = abs_path.parent().unwrap();

        create_dir_all(&abs_parent).unwrap();

        let mut file = File::create(&abs_path).unwrap();
        file.write_all(contents.as_bytes()).unwrap();

        self.index.add_path(Path::new(path)).unwrap();

        self
    }

    pub fn touch(&mut self, path: &str) -> &mut Self {
        self.count += 1;
        self.write(path, &self.count.to_string())
    }

    pub fn commit(&mut self, message: &str) -> &mut Self {
        {
            let oid = self.index.write_tree().unwrap();
            let tree = self.repo.find_tree(oid).unwrap();
            let sig = git2::Signature::now("John Smith", "john@example.com").unwrap();

            self.repo.commit(
                Some("refs/heads/master"),
                &sig,
                &sig,
                "Initial commit",
                &tree,
                &[]).unwrap();
        }

        self
    }

    pub fn initial_commit(&mut self) -> &mut Self {
        self
            .write("Cargo.toml", &manifest("example", "0.1.0"))
            .touch("src/lib.rs")
            .commit("Initial commit")
    }
}

fn manifest(name: &str, version: &str) -> String {
    format!("\
[package]
name = {:?}
version = {:?}
authors = [\"John Smith <jon.smith@example.com>\"]
edition = \"2018\"

[dependencies]
", name, version)
}
