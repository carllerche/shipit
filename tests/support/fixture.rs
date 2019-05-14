use fs_extra::dir::{self, CopyOptions};
use git2::{self, Repository};
use tempdir::TempDir;
use shipit::{Config, Workspace};
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct Fixture {
    /// Path to the workspace
    path: PathBuf,

    /// Git repository handle
    repo: Repository,

    index: git2::Index,

    /// Handle the the temporary directory.
    _tempdir: TempDir,
}

pub fn template(template: &str) -> Fixture {
    let src = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(format!("tests/fixtures/{}", template));

    let tempdir = TempDir::new("shipit-tests").unwrap();
    let path = tempdir.path().join("workspace");

    let opts = CopyOptions {
        copy_inside: true,
        .. CopyOptions::new()
    };

    dir::copy(&src, &path, &opts).unwrap();

    // Initialize git repository
    let repo = Repository::init(&path).unwrap();
    let mut index = repo.index().unwrap();

    // Configure the remote
    repo.remote_add_fetch("origin", "refs/*:refs/*").unwrap();
    repo.remote_set_url("origin", "https://github.com/example/test").unwrap();

    // Add all the files
    for entry in WalkDir::new(&path) {
        let entry = entry.unwrap();

        if entry.path().is_file() {
            let entry_path = entry.path().strip_prefix(&path).unwrap();

            if !entry_path.starts_with(".git") {
                index.add_path(entry_path).unwrap();
            }
        }
    }

    {
        let oid = index.write_tree().unwrap();
        let tree = repo.find_tree(oid).unwrap();
        let sig = git2::Signature::now("John Smith", "john@example.com").unwrap();

        repo.commit(
            Some("refs/heads/master"),
            &sig,
            &sig,
            "Initial commit",
            &tree,
            &[]).unwrap();
    }

    Fixture {
        path,
        repo,
        index,
        _tempdir: tempdir,
    }
}

impl Fixture {
    /// Return the path to the workspace root
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Return the workspace
    pub fn workspace(&self) -> Workspace {
        Workspace::load(&self.path)
    }

    /// Return the workspace configuration
    pub fn config(&self) -> Result<Config, shipit::Error> {
        Config::load(&self.workspace())
    }

    /// Write a file
    pub fn write_file<P>(&self, path: P, contents: &str)
    where
        P: AsRef<Path>,
    {
        let path = self.path.join(path);
        let mut file = File::create(&path).unwrap();

        file.write_all(contents.as_bytes()).unwrap();
    }
}
