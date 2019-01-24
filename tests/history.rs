use chrono::prelude::*;
use shipit::History;
use serde_json::json;

mod support;
use self::support::*;

#[test]
fn smoke() {
    let now = Utc::now();

    let mut git = git::Builder::new();
    let sha = git.initial_commit();

    println!("UPDATED_AT = {}", now.to_string());

    let github = github::Builder::new()
        .response(json!({
            "data": {
                "repository": {
                    "pullRequests": {
                        "edges": [
                            {
                                "cursor": "foo",
                                "node": {
                                    "number": 1,
                                    "updatedAt": now.to_string(),
                                    "mergeCommit": {
                                        "oid": sha.to_string(),
                                    }
                                }
                            }
                        ],
                        "pageInfo": {
                            "hasNextPage": false,
                        }
                    }
                },
            }
        }))
        .build();

    let history = History::load(
        &mut git.repository(),
        &git::Ref::head("master"),
        &[],
        &github);

    assert_eq!(0, history.commits().len());
}
