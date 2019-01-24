use super::{DateTime, Transport};

use graphql_client::{GraphQLQuery, Response};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.json",
    query_path = "graphql/github.graphql",
    response_derives = "Debug",
)]
struct PullRequests;

#[derive(Debug, Clone)]
pub struct PullRequest {
    pub number: u64,
    pub title: String,
    pub updated_at: DateTime,
    pub merge_commit: git2::Oid,
}

pub fn query<'a, T>(client: &'a T, repo: &'a super::RepositoryId)
    -> impl Iterator<Item = Result<PullRequest, super::Error>> + 'a
where
    T: Transport,
{
    Iter {
        client,
        repo,
        after: None,
        done: false,
    }
    .flatten()
}

struct Iter<'a, T> {
    client: &'a T,
    repo: &'a super::RepositoryId,
    after: Option<String>,
    done: bool,
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: Transport,
{
    type Item = Vec<Result<PullRequest, super::Error>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let q = PullRequests::build_query(pull_requests::Variables {
            owner: self.repo.owner.clone(),
            name: self.repo.name.clone(),
            after: self.after.take(),
        });

        let response: Response<pull_requests::ResponseData> =
            match self.client.query(&q) {
                Ok(response) => response,
                Err(e) => {
                    return Some(vec![Err(e.into())]);
                }
            };

        let pull_requests = response
            .data.unwrap()
            .repository.unwrap()
            .pull_requests;

        self.done = !pull_requests.page_info.has_next_page;

        let mut ret = vec![];

        for edge in pull_requests.edges.unwrap() {
            let edge = edge.unwrap();
            self.after = Some(edge.cursor);

            let node = edge.node.unwrap();

            ret.push(Ok(PullRequest {
                number: node.number as u64,
                title: node.title,
                updated_at: node.updated_at,
                merge_commit: node.merge_commit.unwrap().oid.parse().unwrap(),
            }));
        }

        Some(ret)
    }
}

pub type GitObjectID = String;
