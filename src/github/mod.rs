mod pulls;
mod pushed_date;
mod transport;

pub use self::pulls::PullRequest;
pub use self::transport::Transport;

use crate::git;

use chrono::{self, offset::Utc};
use reqwest;
use url::{Host, Url};

/// Issue requests to Github
pub struct Client<T = reqwest::Client> {
    transport: T,
}

/// How Github represents their date & time.
pub type DateTime = chrono::DateTime<Utc>;

/// Error type
pub type Error = Box<dyn ::std::error::Error>;

pub struct RepositoryId {
    owner: String,
    name: String,
}

impl<T> Client<T>
where
    T: Transport,
{
    pub fn with_transport(transport: T) -> Client<T> {
        Client {
            transport,
        }
    }

    /// Find the oldest published date for the commits referenced by `refs`.
    pub fn pushed_date(&self, repo: &RepositoryId, refs: &[git::Ref])
        -> Result<DateTime, Error>
    {
        pushed_date::query(&self.transport, repo, refs)
    }

    /// Get pull requests
    pub fn pull_requests<'a>(&'a self, repo: &'a RepositoryId)
        -> impl Iterator<Item = Result<PullRequest, Error>> + 'a
    {
        pulls::query(&self.transport, repo)
    }
}

impl RepositoryId {
    pub fn from_url(url: &Url) -> RepositoryId {
        assert_eq!(url.host(), Some(Host::Domain("github.com")));

        let segments: Vec<_> = url.path_segments()
            .expect("invalid Github repository URL")
            .collect();

        RepositoryId {
            owner: segments[0].to_string(),
            name: segments[1].to_string(),
        }
    }
}

/*
struct PullIter<'a> {
    http: &'a reqwest::Client,
    next: Option<String>,
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/github.json",
    query_path = "graphql/my_query.graphql",
    response_derives = "Debug",
)]
struct TestQuery;

pub fn graphql_wut(config: &config::System) {
    let q = TestQuery::build_query(test_query::Variables {});

    let http = reqwest::Client::new();

    let mut res = http
        .post("https://api.github.com/graphql")
        .bearer_auth(config.github_token.as_ref().unwrap())
        .json(&q)
        .send()
        .unwrap();

    let response_body: Response<test_query::ResponseData> = res.json().unwrap();
    println!("{:#?}", response_body);
}

impl Client {
    pub fn new(config: &config::System) -> Client {
        use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
        let mut authentication = HeaderMap::new();

        if let Some(ref token) = config.github_token {
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

    pub fn merged_pull_requests<'a>(&'a self) -> impl Iterator<Item = Pull> + 'a {
        PullIter {
            http: &self.http,
            next: Some("https://api.github.com/repos/tokio-rs/tokio/pulls?state=closed&per_page=30&sort=updated&direction=desc".to_string())
        }
        .flatten()
        .filter(|pr| pr.merge_commit_sha.is_some())
    }
}

impl<'a> Iterator for PullIter<'a> {
    type Item = Vec<Pull>;

    fn next(&mut self) -> Option<Vec<Pull>> {
        let next = match self.next.take() {
            Some(next) => next,
            None => return None,
        };

        let mut response = self.http.get(&next).send().unwrap();

        if let Some(hdr) = response.headers().get("link") {
            // Very hacky way to extract the next link
            let re = regex::Regex::new(r#"<([^>]+)>;\s*rel="next""#).unwrap();

            if let Some(captures) = re.captures(hdr.to_str().unwrap()) {
                let url = captures.get(1).unwrap().as_str();
                self.next = Some(url.to_string());
            }
        }

        Some(response.json().unwrap())
    }
}
*/
