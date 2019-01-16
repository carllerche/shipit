use std::env;

#[derive(Debug)]
pub struct System {
    pub github_token: Option<String>,
}

const GITHUB_TOKEN_KEY: &str = "SHIPIT_GITHUB_TOKEN";

impl System {
    pub fn load() -> System {
        System {
            github_token: env::var(GITHUB_TOKEN_KEY).ok(),
        }
    }
}
