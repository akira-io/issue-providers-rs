use issue_provider_core::{
    BoxFuture, Cycle, CycleId, ErrorKind, IssueResult, Label, LabelId, Milestone, MilestoneId,
    Page, PageCursor, PageRequest, Project, ProjectId, Team, TeamId, User, UserId, error,
};
use serde::Deserialize;

use crate::client::LinearClient;

#[derive(Deserialize)]
struct NamedNode {
    id: String,
    name: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct PageInfo {
    has_next_page: bool,
    end_cursor: Option<String>,
}

#[derive(Deserialize)]
struct NamedConnection {
    nodes: Vec<NamedNode>,
    #[serde(rename = "pageInfo")]
    page_info: PageInfo,
}

fn decode<T: serde::de::DeserializeOwned>(value: serde_json::Value) -> IssueResult<T> {
    serde_json::from_value(value)
        .map_err(|err| error().of(ErrorKind::Decode, format!("decode: {err}")))
}

macro_rules! named_capability {
    ($trait_name:ident, $entity:ident, $id:ty, $single:literal, $list:literal, $not_found:literal) => {
        impl issue_provider_core::$trait_name for LinearClient {
            fn get(&self, id: $id) -> BoxFuture<'_, IssueResult<$entity>> {
                Box::pin(async move {
                    let query = concat!(
                        "query($id: String!) { ",
                        $single,
                        "(id: $id) { id name } }"
                    );
                    let raw: serde_json::Value = self
                        .execute(query, serde_json::json!({ "id": id.as_str() }))
                        .await?;
                    let node = raw.get($single).cloned().unwrap_or(serde_json::Value::Null);
                    if node.is_null() {
                        return Err(error().of(ErrorKind::NotFound, $not_found));
                    }
                    let node: NamedNode = decode(node)?;
                    Ok($entity::make(<$id>::make(node.id), node.name.unwrap_or_default()))
                })
            }

            fn list(
                &self,
                page: Option<PageRequest>,
            ) -> BoxFuture<'_, IssueResult<Page<$entity>>> {
                Box::pin(async move {
                    let first = page.as_ref().and_then(PageRequest::limit).unwrap_or(50);
                    let after = page
                        .as_ref()
                        .and_then(PageRequest::after)
                        .map(|cursor| cursor.as_str().to_string());

                    let query = concat!(
                        "query($first: Int!, $after: String) { ",
                        $list,
                        "(first: $first, after: $after) { nodes { id name } pageInfo { hasNextPage endCursor } } }"
                    );
                    let raw: serde_json::Value = self
                        .execute(query, serde_json::json!({ "first": first, "after": after }))
                        .await?;
                    let connection: NamedConnection =
                        decode(raw.get($list).cloned().unwrap_or(serde_json::Value::Null))?;

                    let items = connection
                        .nodes
                        .into_iter()
                        .map(|node| {
                            $entity::make(<$id>::make(node.id), node.name.unwrap_or_default())
                        })
                        .collect();
                    let next = if connection.page_info.has_next_page {
                        connection.page_info.end_cursor.map(PageCursor::make)
                    } else {
                        None
                    };

                    Ok(Page::make(items, next))
                })
            }
        }
    };
}

named_capability!(
    Projects,
    Project,
    ProjectId,
    "project",
    "projects",
    "linear project not found"
);
named_capability!(
    Milestones,
    Milestone,
    MilestoneId,
    "projectMilestone",
    "projectMilestones",
    "linear milestone not found"
);
named_capability!(
    Cycles,
    Cycle,
    CycleId,
    "cycle",
    "cycles",
    "linear cycle not found"
);
named_capability!(
    Teams,
    Team,
    TeamId,
    "team",
    "teams",
    "linear team not found"
);
named_capability!(
    Users,
    User,
    UserId,
    "user",
    "users",
    "linear user not found"
);
named_capability!(
    Labels,
    Label,
    LabelId,
    "issueLabel",
    "issueLabels",
    "linear label not found"
);
