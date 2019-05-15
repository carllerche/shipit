mod pulls;
// mod pushed_date;
mod transport;

use crate::config;
use self::transport::Transport;

use chrono::{self, offset::Utc};
use url::{Host, Url};

/// Issue requests to Github
pub struct Client {
    transport: Transport,
}

/// How Github represents their date & time.
pub type DateTime = chrono::DateTime<Utc>;

/// Error type
pub type Error = Box<dyn ::std::error::Error>;

pub struct RepositoryId {
    owner: String,
    name: String,
}

impl Client {
    pub fn new(config: &config::System) -> Client {
        let transport = Transport::new(config);
        Client { transport }
    }

    pub fn associated_prs<'a>(&self, repo: &RepositoryId, commits: impl Iterator<Item = &'a git2::Oid>) -> Result<(), Error> {
        pulls::query(&self.transport, repo, commits)
    }
}

impl RepositoryId {
    pub fn from_url(url: &Url) -> RepositoryId {
        assert_eq!(url.host(), Some(Host::Domain("github.com")));

        let segments: Vec<_> = url
            .path_segments()
            .expect("invalid Github repository URL")
            .collect();

        RepositoryId {
            owner: segments[0].to_string(),
            name: segments[1].to_string(),
        }
    }
}
