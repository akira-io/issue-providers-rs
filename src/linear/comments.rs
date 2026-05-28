use crate::{
    BoxFuture, Comment, CommentId, Comments, ErrorKind, IssueId, IssueResult, Page, PageCursor,
    PageRequest, comment, error,
};
use serde::Deserialize;

use super::client::LinearClient;

const COMMENT_FIELDS: &str = "id body createdAt user { id }";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CommentNode {
    id: String,
    body: String,
    created_at: Option<String>,
    user: Option<RefNode>,
}

#[derive(Deserialize)]
struct RefNode {
    id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PageInfo {
    has_next_page: bool,
    end_cursor: Option<String>,
}

fn map_comment(node: CommentNode) -> Comment {
    let mut builder = comment().id(node.id).body(node.body);
    if let Some(user) = node.user {
        builder = builder.author(user.id);
    }
    if let Some(created_at) = node.created_at {
        builder = builder.created_at(created_at);
    }
    builder.build()
}

impl Comments for LinearClient {
    fn list_comments(
        &self,
        issue: IssueId,
        page: Option<PageRequest>,
    ) -> BoxFuture<'_, IssueResult<Page<Comment>>> {
        Box::pin(async move {
            #[derive(Deserialize)]
            struct Data {
                issue: Option<IssueComments>,
            }
            #[derive(Deserialize)]
            struct IssueComments {
                comments: Connection,
            }
            #[derive(Deserialize)]
            struct Connection {
                nodes: Vec<CommentNode>,
                #[serde(rename = "pageInfo")]
                page_info: PageInfo,
            }

            let first = page.as_ref().and_then(PageRequest::limit).unwrap_or(50);
            let after = page
                .as_ref()
                .and_then(PageRequest::after)
                .map(|cursor| cursor.as_str().to_string());

            let query = format!(
                "query($id: String!, $first: Int!, $after: String) {{ issue(id: $id) {{ comments(first: $first, after: $after) {{ nodes {{ {COMMENT_FIELDS} }} pageInfo {{ hasNextPage endCursor }} }} }} }}"
            );
            let data: Data = self
                .execute(
                    &query,
                    serde_json::json!({ "id": issue.as_str(), "first": first, "after": after }),
                )
                .await?;

            let connection = data
                .issue
                .ok_or_else(|| error().of(ErrorKind::NotFound, "linear issue not found"))?
                .comments;

            let items = connection.nodes.into_iter().map(map_comment).collect();
            let next = if connection.page_info.has_next_page {
                connection.page_info.end_cursor.map(PageCursor::make)
            } else {
                None
            };

            Ok(Page::make(items, next))
        })
    }

    fn post_comment(&self, issue: IssueId, body: String) -> BoxFuture<'_, IssueResult<Comment>> {
        Box::pin(async move {
            #[derive(Deserialize)]
            struct Data {
                #[serde(rename = "commentCreate")]
                result: Mutation,
            }
            #[derive(Deserialize)]
            struct Mutation {
                comment: Option<CommentNode>,
            }

            let query = format!(
                "mutation($input: CommentCreateInput!) {{ commentCreate(input: $input) {{ comment {{ {COMMENT_FIELDS} }} }} }}"
            );
            let data: Data = self
                .execute(
                    &query,
                    serde_json::json!({ "input": { "issueId": issue.as_str(), "body": body } }),
                )
                .await?;

            data.result.comment.map(map_comment).ok_or_else(|| {
                error().of(
                    ErrorKind::Provider,
                    "linear commentCreate returned no comment",
                )
            })
        })
    }

    fn delete_comment(&self, id: CommentId) -> BoxFuture<'_, IssueResult<()>> {
        Box::pin(async move {
            #[derive(Deserialize)]
            struct Data {
                #[serde(rename = "commentDelete")]
                result: Mutation,
            }
            #[derive(Deserialize)]
            struct Mutation {
                success: bool,
            }

            let query = "mutation($id: String!) { commentDelete(id: $id) { success } }";
            let data: Data = self
                .execute(query, serde_json::json!({ "id": id.as_str() }))
                .await?;

            if data.result.success {
                Ok(())
            } else {
                Err(error().of(ErrorKind::Provider, "linear commentDelete reported failure"))
            }
        })
    }
}
