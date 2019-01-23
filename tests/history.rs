use shipit::History;
use serde_json::json;

mod support;
use self::support::*;

#[test]
fn smoke() {
    let mut git = git::Builder::new();
    git.initial_commit();

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
