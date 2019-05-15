use crate::github::{Error, Transport};

use serde_derive::{Serialize, Deserialize};
use std::collections::HashMap;

pub fn query<'a>(
    transport: &Transport,
    repo: &super::RepositoryId,
    commits: impl Iterator<Item = &'a git2::Oid>,
) -> Result<(), Error> {
    use std::fmt::Write;

    let commit_query = commits
        .enumerate()
        .fold(String::new(), |mut s, (i, commit)| {
            write!(s, r##"
                commit_{}: object(oid: "{}") {{
                    ... pr
                }}"##, i, commit).unwrap();
            s
        });


    let query = format!(r##"
        query {{
            repository(owner: {:?}, name: {:?}) {{
                {}
            }}
        }}

        fragment pr on Commit {{
            associatedPullRequests(first:5) {{
                edges {{
                    node {{
                        number
                        title
                        labels(first:5) {{
                            edges {{
                                node {{
                                    name,
                                }}
                            }}
                        }}
                    }}
                }}
            }}
        }}"##, repo.owner, repo.name, commit_query);

    let response: Response = transport.query(&Request {
        query,
    })?;

    println!("{:#?}", response);

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
    // repository: HashMap<String, HashMap<String, serde_json::Value>>,
    repository: HashMap<String, Commit>,
}

#[derive(Debug, Deserialize)]
struct Commit {
    associatedPullRequests: AssociatedPullRequests,
}

#[derive(Debug, Deserialize)]
struct AssociatedPullRequests {
    // edges: serde_json::Value,
    edges: Vec<AssociatedPullRequest>,
}

#[derive(Debug, Deserialize)]
struct AssociatedPullRequest {
    node: serde_json::Value,
    // node: PullRequest,
}

#[derive(Debug, Deserialize)]
struct PullRequest {
    number: u64,
    title: String,
}

/*
#[derive(Debug, Deserialize)]
struct Ref {
    target: Target,
}

#[derive(Debug, Deserialize)]
struct Target {
    pushedDate: super::DateTime,
}
*/
