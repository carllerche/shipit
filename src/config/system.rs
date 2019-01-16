
#[derive(Debug)]
pub struct System {
    pub github_token: Option<String>,
}

impl System {
    pub fn load() -> System {
        System {
            github_token: None,
        }
    }
}
