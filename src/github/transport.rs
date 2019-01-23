use super::Error;
use crate::config;
use reqwest;
use serde::{Serialize};
use serde::de::DeserializeOwned;

pub trait Transport {
    fn query<T, U>(&self, query: &T) -> Result<U, Error>
    where
        T: Serialize,
        U: DeserializeOwned;
}

const QUERY_URL: &str = "https://api.github.com/graphql";

impl super::Client<reqwest::Client> {
    pub fn new(config: &config::System) -> super::Client {
        use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};

        let mut authorization = HeaderMap::new();

        if let Some(ref token) = config.github_token {
            let header_value = format!("Bearer {}", token);
            authorization.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&header_value).unwrap());
        }

        let http = reqwest::ClientBuilder::new()
            .default_headers(authorization)
            .build()
            .unwrap();

        super::Client {
            transport: http
        }
    }
}

impl Transport for reqwest::Client {
    fn query<T, U>(&self, query: &T) -> Result<U, Error>
    where
        T: Serialize,
        U: DeserializeOwned
    {
        let mut response = self
            .post(QUERY_URL)
            .json(&query)
            .send()?;

        Ok(response.json()?)
    }
}
