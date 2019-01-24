use chrono::prelude::*;
use shipit::History;
use serde_json::json;

mod support;
use self::support::*;

#[test]
fn initial_repo() {
    let now = Utc::now();

    let mut git = git::Builder::new();
    let sha = git.initial_commit();

    let github = github::Builder::new()
        .response(json!({
            "data": {
                "repository": {
                    "pullRequests": {
                        "edges": [
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

#[test]
fn single_pull() {
    let now = Utc::now();

    let mut git = git::Builder::new();
    let sha = git.initial_commit();

    let github = github::Builder::new()
        .response(json!({
            "data": {
                "repository": {
                    "pullRequests": {
                        "edges": [
                            {
                                "cursor": "foo",
                                "node": {
                                    "id": "abc",
                                    "number": 123,
                                    "title": "Hello",
                                    "updatedAt": now,
                                    "mergeCommit": {
                                        "oid": sha,
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

    assert_eq!(1, history.commits().len());

    let commit = &history.commits()[0];
    assert_eq!(sha, commit.oid);

    assert_eq!(1, commit.pulls.len());
    assert_eq!(123, commit.pulls[0].number);
}
