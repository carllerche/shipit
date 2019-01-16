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
    pub fn new() -> Client {
        use reqwest::header::{self, HeaderMap, HeaderName, HeaderValue};
        let mut authentication = HeaderMap::new();

        authentication.insert(
            HeaderName::from_static("token"),
            HeaderValue::from_static("dda5c2fe7e0c2da2ad91da6322970b496fb113d9"),
        );

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
