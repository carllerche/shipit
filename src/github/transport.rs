use super::Error;
use crate::config;
use reqwest;
use serde::{Serialize};
use serde::de::DeserializeOwned;
use std::fmt;

pub struct Transport {
    http: reqwest::Client,
}

const QUERY_URL: &str = "https://api.github.com/graphql";

impl Transport {
    pub fn new(config: &config::System) -> Transport {
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

        Transport {
            http: http
        }
    }

    pub fn query<T, U>(&self, query: &T) -> Result<U, Error>
    where
        T: Serialize + fmt::Debug,
        U: DeserializeOwned
    {
        let mut response = self
            .http
            .post(QUERY_URL)
            .json(&query)
            .send()?;

        if !response.status().is_success() {
            println!("ERR\n{}", response.text()?);
            unimplemented!();
        }

        Ok(response.json()?)
    }
}
