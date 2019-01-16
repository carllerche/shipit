use crate::config;

use reqwest;
use serde_derive::Deserialize;

pub struct Client {
    http: reqwest::Client,
}

#[derive(Debug, Deserialize)]
pub struct Pull {
    id: u64,
    url: String,
    title: String,
    merge_commit_sha: Option<String>,
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

    pub fn prs(&self) -> Vec<Pull> {
        let pulls: Vec<Pull> = self
            .http
            .get("https://api.github.com/repos/carllerche/h2/pulls?state=closed")
            .send()
            .unwrap()
            .json()
            .unwrap();

        pulls
    }
}
