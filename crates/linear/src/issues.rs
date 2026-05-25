use issue_provider_core::{
    BoxFuture, ErrorKind, Issue, IssueId, IssueResult, Issues, Page, PageCursor, PageRequest,
    StatusCategory, error, issue,
};
use serde::Deserialize;

use crate::client::LinearClient;

const ISSUE_FIELDS: &str =
    "id title priority updatedAt state { name type } project { id } assignee { id }";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct IssueNode {
    id: String,
    title: String,
    priority: Option<f64>,
    updated_at: Option<String>,
    state: Option<StateNode>,
    project: Option<RefNode>,
    assignee: Option<RefNode>,
}

#[derive(Deserialize)]
struct StateNode {
    name: String,
    #[serde(rename = "type")]
    kind: String,
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

/// Map a Linear workflow-state `type` to the neutral [`StatusCategory`].
pub fn category_from_state_type(kind: &str) -> Option<StatusCategory> {
    Some(match kind {
        "triage" | "backlog" => StatusCategory::Backlog,
        "unstarted" => StatusCategory::Unstarted,
        "started" => StatusCategory::Started,
        "completed" => StatusCategory::Completed,
        "canceled" | "cancelled" => StatusCategory::Canceled,
        _ => return None,
    })
}

fn map_issue(node: IssueNode) -> Issue {
    let status = node
        .state
        .as_ref()
        .map(|state| state.name.clone())
        .unwrap_or_default();
    let mut builder = issue().id(node.id).title(node.title).status(status);

    if let Some(category) = node
        .state
        .as_ref()
        .and_then(|state| category_from_state_type(&state.kind))
    {
        builder = builder.category(category);
    }
    if let Some(project) = node.project {
        builder = builder.project(project.id);
    }
    if let Some(assignee) = node.assignee {
        builder = builder.assignee(assignee.id);
    }
    if let Some(priority) = node.priority {
        builder = builder.priority(priority as u8);
    }
    if let Some(updated_at) = node.updated_at {
        builder = builder.updated_at(updated_at);
    }

    builder.build()
}

impl Issues for LinearClient {
    fn get(&self, id: IssueId) -> BoxFuture<'_, IssueResult<Issue>> {
        Box::pin(async move {
            #[derive(Deserialize)]
            struct Data {
                issue: Option<IssueNode>,
            }

            let query = format!("query($id: String!) {{ issue(id: $id) {{ {ISSUE_FIELDS} }} }}");
            let data: Data = self
                .execute(&query, serde_json::json!({ "id": id.as_str() }))
                .await?;

            data.issue
                .map(map_issue)
                .ok_or_else(|| error().of(ErrorKind::NotFound, "linear issue not found"))
        })
    }

    fn list(&self, page: Option<PageRequest>) -> BoxFuture<'_, IssueResult<Page<Issue>>> {
        Box::pin(async move {
            #[derive(Deserialize)]
            struct Data {
                issues: Connection,
            }
            #[derive(Deserialize)]
            struct Connection {
                nodes: Vec<IssueNode>,
                #[serde(rename = "pageInfo")]
                page_info: PageInfo,
            }

            let first = page.as_ref().and_then(PageRequest::limit).unwrap_or(50);
            let after = page
                .as_ref()
                .and_then(PageRequest::after)
                .map(|cursor| cursor.as_str().to_string());

            let query = format!(
                "query($first: Int!, $after: String) {{ issues(first: $first, after: $after) {{ nodes {{ {ISSUE_FIELDS} }} pageInfo {{ hasNextPage endCursor }} }} }}"
            );
            let data: Data = self
                .execute(
                    &query,
                    serde_json::json!({ "first": first, "after": after }),
                )
                .await?;

            let items = data.issues.nodes.into_iter().map(map_issue).collect();
            let next = if data.issues.page_info.has_next_page {
                data.issues.page_info.end_cursor.map(PageCursor::make)
            } else {
                None
            };

            Ok(Page::make(items, next))
        })
    }
}
