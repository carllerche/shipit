use crate::github::Error;
// use serde_derive::{Serialize, Deserialize};

// use std::collections::HashMap;

pub fn query(_client: &reqwest::Client, _repo: &super::RepositoryId, _commits: &[git2::Oid])
    -> Result<(), Error>
{
    unimplemented!();
    /*
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
            repository(owner: {:?}, name: {:?}) {{
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
        .filter(|r| r.is_some())
        .map(|r| {
            r.as_ref().unwrap().target.pushedDate
        })
        .collect();

    times.sort();

    Ok(times[0])
    */
}

/*
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
    repository: HashMap<String, Option<Ref>>,
}

#[derive(Debug, Deserialize)]
struct Ref {
    target: Target,
}

#[derive(Debug, Deserialize)]
struct Target {
    pushedDate: super::DateTime,
}
*/
