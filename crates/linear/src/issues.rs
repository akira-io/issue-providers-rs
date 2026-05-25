use issue_provider_core::{
    BoxFuture, ErrorKind, Issue, IssueDraft, IssueFilter, IssueId, IssuePatch, IssueResult, Issues,
    Page, PageCursor, PageRequest, StatusCategory, error, issue,
};
use serde::Deserialize;

use crate::client::LinearClient;

fn state_type_for(category: StatusCategory) -> &'static str {
    match category {
        StatusCategory::Backlog => "backlog",
        StatusCategory::Unstarted => "unstarted",
        StatusCategory::Started => "started",
        StatusCategory::Completed => "completed",
        StatusCategory::Canceled => "canceled",
    }
}

const ISSUE_FIELDS: &str = "id identifier title description url priority createdAt updatedAt state { name type } project { id } assignee { id } creator { id } team { id } labels { nodes { id } }";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct IssueNode {
    id: String,
    identifier: Option<String>,
    title: String,
    description: Option<String>,
    url: Option<String>,
    priority: Option<f64>,
    created_at: Option<String>,
    updated_at: Option<String>,
    state: Option<StateNode>,
    project: Option<RefNode>,
    assignee: Option<RefNode>,
    creator: Option<RefNode>,
    team: Option<RefNode>,
    labels: Option<LabelConnection>,
}

#[derive(Deserialize)]
struct LabelConnection {
    nodes: Vec<RefNode>,
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
    if let Some(creator) = node.creator {
        builder = builder.author(creator.id);
    }
    if let Some(team) = node.team {
        builder = builder.team(team.id);
    }
    if let Some(labels) = node.labels {
        builder = builder.labels(labels.nodes.into_iter().map(|label| label.id));
    }
    if let Some(priority) = node.priority {
        builder = builder.priority(priority as u8);
    }
    if let Some(identifier) = node.identifier {
        builder = builder.identifier(identifier);
    }
    if let Some(description) = node.description {
        builder = builder.description(description);
    }
    if let Some(url) = node.url {
        builder = builder.url(url);
    }
    if let Some(created_at) = node.created_at {
        builder = builder.created_at(created_at);
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

    fn list(
        &self,
        filter: IssueFilter,
        page: Option<PageRequest>,
    ) -> BoxFuture<'_, IssueResult<Page<Issue>>> {
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

            let mut linear_filter = serde_json::Map::new();
            if let Some(team) = filter.team() {
                linear_filter.insert(
                    "team".into(),
                    serde_json::json!({ "id": { "eq": team.as_str() } }),
                );
            }
            if let Some(project) = filter.project() {
                linear_filter.insert(
                    "project".into(),
                    serde_json::json!({ "id": { "eq": project.as_str() } }),
                );
            }
            if let Some(assignee) = filter.assignee() {
                linear_filter.insert(
                    "assignee".into(),
                    serde_json::json!({ "id": { "eq": assignee.as_str() } }),
                );
            }
            if let Some(category) = filter.category() {
                linear_filter.insert(
                    "state".into(),
                    serde_json::json!({ "type": { "eq": state_type_for(category) } }),
                );
            }

            let query = format!(
                "query($first: Int!, $after: String, $filter: IssueFilter) {{ issues(first: $first, after: $after, filter: $filter) {{ nodes {{ {ISSUE_FIELDS} }} pageInfo {{ hasNextPage endCursor }} }} }}"
            );
            let data: Data = self
                .execute(
                    &query,
                    serde_json::json!({ "first": first, "after": after, "filter": linear_filter }),
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

    fn create(&self, draft: IssueDraft) -> BoxFuture<'_, IssueResult<Issue>> {
        Box::pin(async move {
            let mut input = serde_json::Map::new();
            input.insert("teamId".into(), draft.team().as_str().into());
            input.insert("title".into(), draft.title().into());
            if let Some(project) = draft.project() {
                input.insert("projectId".into(), project.as_str().into());
            }
            if let Some(milestone) = draft.milestone() {
                input.insert("projectMilestoneId".into(), milestone.as_str().into());
            }
            if let Some(assignee) = draft.assignee() {
                input.insert("assigneeId".into(), assignee.as_str().into());
            }
            if let Some(priority) = draft.priority() {
                input.insert("priority".into(), priority.into());
            }
            if let Some(category) = draft.category() {
                let state_id = self
                    .resolve_state_id(draft.team().as_str(), category)
                    .await?;
                input.insert("stateId".into(), state_id.into());
            }

            #[derive(Deserialize)]
            struct Data {
                #[serde(rename = "issueCreate")]
                result: Mutation,
            }
            #[derive(Deserialize)]
            struct Mutation {
                issue: Option<IssueNode>,
            }

            let query = format!(
                "mutation($input: IssueCreateInput!) {{ issueCreate(input: $input) {{ issue {{ {ISSUE_FIELDS} }} }} }}"
            );
            let data: Data = self
                .execute(&query, serde_json::json!({ "input": input }))
                .await?;

            data.result.issue.map(map_issue).ok_or_else(|| {
                error().of(ErrorKind::Provider, "linear issueCreate returned no issue")
            })
        })
    }

    fn update(&self, id: IssueId, patch: IssuePatch) -> BoxFuture<'_, IssueResult<Issue>> {
        Box::pin(async move {
            if patch.is_empty() {
                return self.get(id).await;
            }

            let mut input = serde_json::Map::new();
            if let Some(title) = patch.title() {
                input.insert("title".into(), title.into());
            }
            if let Some(project) = patch.project() {
                input.insert("projectId".into(), project.as_str().into());
            }
            if let Some(milestone) = patch.milestone() {
                input.insert("projectMilestoneId".into(), milestone.as_str().into());
            }
            if let Some(assignee) = patch.assignee() {
                input.insert("assigneeId".into(), assignee.as_str().into());
            }
            if let Some(priority) = patch.priority() {
                input.insert("priority".into(), priority.into());
            }
            if let Some(category) = patch.category() {
                let team_id = self.issue_team_id(id.as_str()).await?;
                let state_id = self.resolve_state_id(&team_id, category).await?;
                input.insert("stateId".into(), state_id.into());
            }

            #[derive(Deserialize)]
            struct Data {
                #[serde(rename = "issueUpdate")]
                result: Mutation,
            }
            #[derive(Deserialize)]
            struct Mutation {
                issue: Option<IssueNode>,
            }

            let query = format!(
                "mutation($id: String!, $input: IssueUpdateInput!) {{ issueUpdate(id: $id, input: $input) {{ issue {{ {ISSUE_FIELDS} }} }} }}"
            );
            let data: Data = self
                .execute(
                    &query,
                    serde_json::json!({ "id": id.as_str(), "input": input }),
                )
                .await?;

            data.result.issue.map(map_issue).ok_or_else(|| {
                error().of(ErrorKind::Provider, "linear issueUpdate returned no issue")
            })
        })
    }

    fn delete(&self, id: IssueId) -> BoxFuture<'_, IssueResult<()>> {
        Box::pin(async move {
            #[derive(Deserialize)]
            struct Data {
                #[serde(rename = "issueDelete")]
                result: Mutation,
            }
            #[derive(Deserialize)]
            struct Mutation {
                success: bool,
            }

            let query = "mutation($id: String!) { issueDelete(id: $id) { success } }".to_string();
            let data: Data = self
                .execute(&query, serde_json::json!({ "id": id.as_str() }))
                .await?;

            if data.result.success {
                Ok(())
            } else {
                Err(error().of(ErrorKind::Provider, "linear issueDelete reported failure"))
            }
        })
    }
}

impl LinearClient {
    async fn issue_team_id(&self, id: &str) -> IssueResult<String> {
        #[derive(Deserialize)]
        struct Data {
            issue: Option<IssueRef>,
        }
        #[derive(Deserialize)]
        struct IssueRef {
            team: Option<RefNode>,
        }

        let query = "query($id: String!) { issue(id: $id) { team { id } } }";
        let data: Data = self.execute(query, serde_json::json!({ "id": id })).await?;

        data.issue
            .and_then(|issue| issue.team)
            .map(|team| team.id)
            .ok_or_else(|| error().of(ErrorKind::NotFound, "linear issue or team not found"))
    }

    async fn resolve_state_id(
        &self,
        team_id: &str,
        category: StatusCategory,
    ) -> IssueResult<String> {
        #[derive(Deserialize)]
        struct Data {
            team: Option<TeamRef>,
        }
        #[derive(Deserialize)]
        struct TeamRef {
            states: States,
        }
        #[derive(Deserialize)]
        struct States {
            nodes: Vec<StateRef>,
        }
        #[derive(Deserialize)]
        struct StateRef {
            id: String,
            #[serde(rename = "type")]
            kind: String,
        }

        let wanted = state_type_for(category);
        let query = "query($id: String!) { team(id: $id) { states { nodes { id type } } } }";
        let data: Data = self
            .execute(query, serde_json::json!({ "id": team_id }))
            .await?;

        data.team
            .map(|team| team.states.nodes)
            .unwrap_or_default()
            .into_iter()
            .find(|state| state.kind == wanted)
            .map(|state| state.id)
            .ok_or_else(|| {
                error().of(
                    ErrorKind::Provider,
                    format!("linear team has no workflow state of type {wanted}"),
                )
            })
    }
}
