use crate::{
    BoxFuture, Comment, CommentId, Comments, ErrorKind, IssueId, IssueResult, Page, PageRequest,
    comment, error,
};
use reqwest::Method;
use serde::Deserialize;

use super::client::JiraClient;
use super::util::{adf_text, max_results, offset_cursor, start_at, to_adf};

#[derive(Deserialize)]
struct CommentNode {
    id: String,
    body: Option<serde_json::Value>,
    author: Option<AccountNode>,
    created: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AccountNode {
    account_id: Option<String>,
}

fn map_comment(issue_key: &str, node: CommentNode) -> Comment {
    let composite = format!("{issue_key}/{}", node.id);
    let body = node.body.as_ref().map(adf_text).unwrap_or_default();
    let mut builder = comment().id(composite).body(body);
    if let Some(author) = node.author.and_then(|account| account.account_id) {
        builder = builder.author(author);
    }
    if let Some(created) = node.created {
        builder = builder.created_at(created);
    }
    builder.build()
}

impl Comments for JiraClient {
    fn list_comments(
        &self,
        issue: IssueId,
        page: Option<PageRequest>,
    ) -> BoxFuture<'_, IssueResult<Page<Comment>>> {
        Box::pin(async move {
            #[derive(Deserialize)]
            struct Response {
                #[serde(default)]
                comments: Vec<CommentNode>,
            }

            let start = start_at(&page);
            let max = max_results(&page);
            let path = format!(
                "/rest/api/3/issue/{}/comment?startAt={start}&maxResults={max}",
                issue.as_str()
            );
            let data: Response = self.request(Method::GET, &path, None).await?;

            let count = data.comments.len() as u32;
            let items = data
                .comments
                .into_iter()
                .map(|node| map_comment(issue.as_str(), node))
                .collect();
            let next = offset_cursor(None, start, count, max);

            Ok(Page::make(items, next))
        })
    }

    fn post_comment(&self, issue: IssueId, body: String) -> BoxFuture<'_, IssueResult<Comment>> {
        Box::pin(async move {
            let request_body = serde_json::json!({ "body": to_adf(&body) });
            let path = format!("/rest/api/3/issue/{}/comment", issue.as_str());
            let node: CommentNode = self
                .request(Method::POST, &path, Some(&request_body))
                .await?;
            Ok(map_comment(issue.as_str(), node))
        })
    }

    fn delete_comment(&self, id: CommentId) -> BoxFuture<'_, IssueResult<()>> {
        Box::pin(async move {
            let (issue_key, comment_id) = id.as_str().split_once('/').ok_or_else(|| {
                error().of(
                    ErrorKind::Provider,
                    "jira comment id must be in '<issueKey>/<commentId>' form",
                )
            })?;
            let path = format!("/rest/api/3/issue/{issue_key}/comment/{comment_id}");
            self.request_unit(Method::DELETE, &path, None).await
        })
    }
}
