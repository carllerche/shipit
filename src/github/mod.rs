mod pulls;
// mod pushed_date;
// mod transport;

use crate::config;

use chrono::{self, offset::Utc};
use reqwest;
use url::{Host, Url};

/// Issue requests to Github
pub struct Client {
    http: reqwest::Client,
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
        use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
        let mut authentication = HeaderMap::new();

        if let Some(token) = &config.github_token {
            authentication.insert(
                HeaderName::from_static("token"),
                HeaderValue::from_str(token).unwrap(),
            );
        }

        let http = reqwest::ClientBuilder::new()
            .default_headers(authentication)
            .build()
            .unwrap();

        Client { http }
    }

    pub fn associated_prs(&self, repo: &RepositoryId, commits: &[git2::Oid]) -> Result<(), Error> {
        pulls::query(&self.http, repo, commits)
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
