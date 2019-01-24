use super::Transport;
use crate::git;
use serde_derive::{Serialize, Deserialize};

use std::collections::HashMap;

pub fn query<T>(client: &T, repo: &super::RepositoryId, refs: &[git::Ref])
    -> Result<super::DateTime, super::Error>
where
    T: Transport,
{
    // Build the fragments
    let fragments = refs
        .iter()
        .enumerate()
        .map(|(i, r)| {
            format!(r##"
                alias_{}: ref(qualifiedName: "{}") {{
                    target {{
                        ... on Commit {{
                            pushedDate
                        }}
                    }}
                }}
                "##, i, r)
        })
        .fold(String::new(), |mut s, frag| {
            s.push_str(&frag);
            s
        });

    let query = format!(r##"
        query {{
            repository(owner: {}, name: {}) {{
                {}
            }}
        }}"##, repo.owner, repo.name, fragments);

    let response: Response = client.query(&Request {
        query,
    })?;

    let mut times: Vec<_> = response
        .data
        .repository
        .values()
        .map(|r| {
            r.target.pushedDate
        })
        .collect();

    times.sort();

    Ok(times[0])
}

#[derive(Debug, Serialize)]
struct Request {
    query: String,
}

#[derive(Debug, Deserialize)]
struct Response {
    data: Data,
}

#[derive(Debug, Deserialize)]
struct Data {
    repository: HashMap<String, Ref>,
}

#[derive(Debug, Deserialize)]
struct Ref {
    target: Target,
}

#[derive(Debug, Deserialize)]
struct Target {
    pushedDate: super::DateTime,
}
