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
#[derive(Debug)]
pub enum Ref<T> {
    Remote { remote: T, name: T },
    Tag(T),
    Sha(git2::Oid),
}

/// Set of changed files
pub struct FileSet {
    files: Vec<PathBuf>,
}

pub type Error = Box<dyn ::std::error::Error>;

/// Absolute git ref notation
struct Absolute<'a, T>(&'a Ref<T>);

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
    pub fn deref<T>(&self, reference: &Ref<T>) -> Result<git2::Oid, Error>
    where
        T: AsRef<str>
    {
        match *reference {
            Ref::Sha(oid) => {
                // Ensure the commit exists in the repository
                self.inner.find_commit(oid)?;
                Ok(oid)
            }
            _ => {
                let reference = Absolute(reference).to_string();
                let oid = self.inner.refname_to_id(&reference)?;
                Ok(oid)
            }
        }
    }

    /// Returns `true` if the commit referenced by `tag` is contained by the
    /// branch `branch`.
    pub fn is_descendant_of<T, U>(&self, descendant: &Ref<T>, ancestor: &Ref<U>) -> bool
    where T: AsRef<str>,
          U: AsRef<str>,
    {
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

    pub fn find_commit(&self, oid: git2::Oid) -> Result<git2::Commit, Error> {
        self.inner.find_commit(oid)
            .map_err(Into::into)
    }

    pub fn commits_in_range<'a, T, U>(
        &'a self,
        from: &Ref<T>,
        to: &Ref<U>,
    ) -> impl Iterator<Item = git2::Oid> + 'a
    where
        T: AsRef<str>,
        U: AsRef<str>,
    {
        // The tree must be walked
        let mut walk = self.inner.revwalk().unwrap();

        let range = format!("{}..{}", Absolute(from), Absolute(to));

        // Push the requested ranged, formatted appropriately.
        walk.push_range(&range).unwrap();

        walk.map(|res| res.unwrap())
    }

    pub fn merge_base<T>(&self, refs: &[Ref<T>]) -> Result<git2::Oid, Error> {
        assert!(!refs.is_empty(), "empty refs array");

        let iter = refs.iter();
        let first_ref = iter.next().unwrap();

        let mut base = self.deref(first_ref).unwrap();

        for next_ref = iter {
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

impl<'a, T> fmt::Display for Absolute<'a, T>
where
    T: AsRef<str>
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use self::Ref::*;

        match self.0 {
            Remote { remote, name } => write!(fmt, "refs/remotes/{}/{}", remote.as_ref(), name.as_ref()),
            Tag(name) => write!(fmt, "refs/tags/{}", name.as_ref()),
            Sha(oid) => write!(fmt, "{}", oid),
        }
    }
}
