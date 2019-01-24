use crate::config::TagFormat;
use crate::{Package, Workspace};

use git2;
use semver::Version;
use url::{Host, Url};

use std::fmt;
use std::path::{Path, PathBuf};

pub struct Repository {
    /// Git repository handle
    inner: git2::Repository,

    /// Repository tags
    tags: Vec<String>,

    origin_url: Url,
}

/// A git ref.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Ref {
    Remote { remote: String, name: String },
    Head(String),
    Tag(String),
    Sha(git2::Oid),
}

/// Set of changed files
pub struct FileSet {
    files: Vec<PathBuf>,
}

pub type Error = Box<dyn ::std::error::Error>;

impl Repository {
    pub fn open(path: &Path) -> Repository {
        let inner = match git2::Repository::open(path) {
            Ok(v) => v,
            Err(e) => panic!("failed to open: {}", e),
        };

        let tags = inner
            .tag_names(None)
            .unwrap()
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

    /// Returns the `Oid` referenced by `reference`.
    pub fn deref(&self, reference: &Ref) -> Result<git2::Oid, Error> {
        match *reference {
            Ref::Sha(oid) => {
                // Ensure the commit exists in the repository
                self.inner.find_commit(oid)?;
                Ok(oid)
            }
            _ => {
                let reference = reference.to_string();
                let oid = self.inner.refname_to_id(&reference)?;
                Ok(oid)
            }
        }
    }

    pub fn references(&self, reference: &Ref, oid: git2::Oid) -> bool {
        match self.deref(reference) {
            Ok(o) if o == oid => true,
            _ => false,
        }
    }

    /// Returns `true` if the commit referenced by `tag` is contained by the
    /// branch `branch`.
    pub fn is_descendant_of(&self, descendant: &Ref, ancestor: &Ref) -> bool {
        let descendant_id = match self.deref(descendant) {
            Ok(oid) => oid,
            Err(_) => return false,
        };

        let ancestor_id = match self.deref(ancestor) {
            Ok(oid) => oid,
            Err(_) => return false,
        };

        self.inner
            .graph_descendant_of(descendant_id, ancestor_id)
            .unwrap()
    }

    pub fn find_commit(&self, oid: &Ref) -> Result<git2::Commit, Error> {
        let oid = self.deref(oid)?;

        self.inner.find_commit(oid)
            .map_err(Into::into)
    }

    pub fn commits_in_range<'a>(
        &'a self,
        from: &Ref,
        to: &Ref,
    ) -> impl Iterator<Item = git2::Oid> + 'a {
        // The tree must be walked
        let mut walk = self.inner.revwalk().unwrap();

        let range = format!("{}..{}", from, to);

        // Push the requested ranged, formatted appropriately.
        walk.push_range(&range).unwrap();

        walk.map(|res| res.unwrap())
    }

    pub fn merge_base(&self, refs: &[Ref]) -> Result<git2::Oid, Error> {
        assert!(!refs.is_empty(), "empty refs array");

        let mut iter = refs.iter();
        let first_ref = iter.next().unwrap();

        let mut base = self.deref(first_ref).unwrap();

        for next_ref in iter {
            let oid = self.deref(next_ref).unwrap();
            base = self.inner.merge_base(base, oid)?;
        }

        Ok(base)
    }

    pub fn files_changed(&self, commit_id: git2::Oid) -> FileSet {
        let commit = self.inner.find_commit(commit_id).unwrap();

        let tree = commit.tree().unwrap();

        let parent = if commit.parent_count() > 1 {
            None
        } else {
            let parent = commit.parent(0).unwrap();
            Some(parent.tree().unwrap())
        };

        let diff = self
            .inner
            .diff_tree_to_tree(Some(&tree), parent.as_ref(), None)
            .unwrap();

        let mut files = vec![];

        diff.foreach(
            &mut |delta, _progress| {
                if let Some(path) = delta.old_file().path() {
                    files.push(path.to_owned());
                }

                if let Some(path) = delta.new_file().path() {
                    files.push(path.to_owned());
                }

                true
            },
            None,
            None,
            None,
        )
        .unwrap();

        files.dedup();

        FileSet::new(files)
    }
}

pub fn tag_for(name: &str, version: &Version, format: TagFormat) -> String {
    match format {
        TagFormat::VersionOnly => format!("v{}", version),
        TagFormat::NameVersion => format!("{}-{}", name, version),
    }
}

impl FileSet {
    fn new(files: Vec<PathBuf>) -> FileSet {
        FileSet { files }
    }

    /// Returns true if any modified files are part of the specified project
    pub fn modifies(&self, package: &Package, workspace: &Workspace) -> bool {
        'outer: for file in &self.files {
            if !file.starts_with(package.path()) {
                continue;
            }

            // Make sure it isn't contained by another project
            for member in workspace.members() {
                if member.path() == package.path() {
                    // `member` is the current project
                    continue;
                }

                if package.path().starts_with(member.path()) {
                    // `package` is contained by `member`
                    continue;
                }

                if file.starts_with(member.path()) {
                    continue 'outer;
                }
            }

            return true;
        }

        false
    }
}

impl Ref {
    pub fn remote(remote: &str, name: &str) -> Ref {
        Ref::Remote {
            remote: remote.to_string(),
            name: name.to_string(),
        }
    }

    pub fn head(name: &str) -> Ref {
        Ref::Head(name.to_string())
    }

    pub fn tag(name: &str) -> Ref {
        Ref::Tag(name.to_string())
    }
}

impl fmt::Display for Ref {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::Ref::*;

        match *self {
            Remote { ref remote, ref name } => {
                write!(fmt, "refs/remotes/{}/{}", remote, name)
            }
            Head(ref name) => {
                write!(fmt, "refs/heads/{}", name)
            }
            Tag(ref name) => {
                write!(fmt, "refs/tags/{}", name)
            }
            Sha(oid) => write!(fmt, "{}", oid),
        }
    }
}

impl PartialEq<git2::Oid> for Ref {
    fn eq(&self, other: &git2::Oid) -> bool {
        match *self {
            Ref::Sha(oid) => oid == *other,
            _ => false,
        }
    }
}

impl serde::Serialize for Ref {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        self.to_string().serialize(serializer)
    }
}
