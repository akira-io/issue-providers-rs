use crate::{
    BoxFuture, Issue, IssueDraft, IssueFilter, IssueId, IssuePatch, IssueResult, Issues, Page,
    PageCursor, PageRequest, StatusCategory,
};
use reqwest::Method;
use serde::Deserialize;

use super::client::JiraClient;
use super::model::{ISSUE_FIELDS, IssueNode, map_issue};

fn jql_category(category: StatusCategory) -> &'static str {
    match category {
        StatusCategory::Backlog | StatusCategory::Unstarted => "To Do",
        StatusCategory::Started => "In Progress",
        StatusCategory::Completed | StatusCategory::Canceled => "Done",
    }
}

fn escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn build_jql(filter: &IssueFilter) -> String {
    let mut clauses = Vec::new();
    if let Some(project) = filter.project() {
        clauses.push(format!("project = \"{}\"", escape(project.as_str())));
    }
    if let Some(assignee) = filter.assignee() {
        clauses.push(format!("assignee = \"{}\"", escape(assignee.as_str())));
    }
    if let Some(category) = filter.category() {
        clauses.push(format!("statusCategory = \"{}\"", jql_category(category)));
    }

    if clauses.is_empty() {
        clauses.push("created >= \"1970/01/01\"".to_string());
    }

    let mut jql = clauses.join(" AND ");
    jql.push_str(" ORDER BY updated DESC");
    jql
}

impl Issues for JiraClient {
    fn get(&self, id: IssueId) -> BoxFuture<'_, IssueResult<Issue>> {
        Box::pin(async move {
            let path = format!("/rest/api/3/issue/{}?fields={ISSUE_FIELDS}", id.as_str());
            let node: IssueNode = self.request(Method::GET, &path, None).await?;
            Ok(map_issue(node, &self.base_url))
        })
    }

    fn list(
        &self,
        filter: IssueFilter,
        page: Option<PageRequest>,
    ) -> BoxFuture<'_, IssueResult<Page<Issue>>> {
        Box::pin(async move {
            #[derive(Deserialize)]
            struct SearchResponse {
                #[serde(default)]
                issues: Vec<IssueNode>,
                #[serde(rename = "nextPageToken")]
                next_page_token: Option<String>,
                #[serde(rename = "isLast")]
                is_last: Option<bool>,
            }

            let max = page.as_ref().and_then(PageRequest::limit).unwrap_or(50);
            let token = page
                .as_ref()
                .and_then(PageRequest::after)
                .map(|cursor| cursor.as_str().to_string());
            let fields: Vec<&str> = ISSUE_FIELDS.split(',').collect();

            let mut body = serde_json::json!({
                "jql": build_jql(&filter),
                "maxResults": max,
                "fields": fields,
            });
            if let Some(token) = token {
                body["nextPageToken"] = token.into();
            }

            let data: SearchResponse = self
                .request(Method::POST, "/rest/api/3/search/jql", Some(&body))
                .await?;

            let items = data
                .issues
                .into_iter()
                .map(|node| map_issue(node, &self.base_url))
                .collect();
            let next = match data.is_last {
                Some(true) => None,
                _ => data.next_page_token.map(PageCursor::make),
            };

            Ok(Page::make(items, next))
        })
    }

    fn create(&self, draft: IssueDraft) -> BoxFuture<'_, IssueResult<Issue>> {
        Box::pin(async move {
            let project_key = draft
                .project()
                .map(|project| project.as_str().to_string())
                .unwrap_or_else(|| draft.team().as_str().to_string());

            let mut fields = serde_json::Map::new();
            fields.insert("project".into(), serde_json::json!({ "key": project_key }));
            fields.insert("summary".into(), draft.title().into());
            fields.insert("issuetype".into(), serde_json::json!({ "name": "Task" }));
            if let Some(assignee) = draft.assignee() {
                fields.insert(
                    "assignee".into(),
                    serde_json::json!({ "accountId": assignee.as_str() }),
                );
            }
            if let Some(priority) = draft.priority() {
                fields.insert(
                    "priority".into(),
                    serde_json::json!({ "id": priority.to_string() }),
                );
            }
            if let Some(milestone) = draft.milestone() {
                fields.insert(
                    "fixVersions".into(),
                    serde_json::json!([{ "id": milestone.as_str() }]),
                );
            }

            #[derive(Deserialize)]
            struct Created {
                key: String,
            }

            let body = serde_json::json!({ "fields": fields });
            let created: Created = self
                .request(Method::POST, "/rest/api/3/issue", Some(&body))
                .await?;

            match (draft.status(), draft.category()) {
                (Some(status), _) => self.transition_status(&created.key, status).await?,
                (None, Some(category)) => self.transition_category(&created.key, category).await?,
                (None, None) => {}
            }

            self.get(IssueId::make(created.key)).await
        })
    }

    fn update(&self, id: IssueId, patch: IssuePatch) -> BoxFuture<'_, IssueResult<Issue>> {
        Box::pin(async move {
            if patch.is_empty() {
                return self.get(id).await;
            }

            let mut fields = serde_json::Map::new();
            if let Some(title) = patch.title() {
                fields.insert("summary".into(), title.into());
            }
            if let Some(assignee) = patch.assignee() {
                fields.insert(
                    "assignee".into(),
                    serde_json::json!({ "accountId": assignee.as_str() }),
                );
            }
            if let Some(priority) = patch.priority() {
                fields.insert(
                    "priority".into(),
                    serde_json::json!({ "id": priority.to_string() }),
                );
            }
            if let Some(milestone) = patch.milestone() {
                fields.insert(
                    "fixVersions".into(),
                    serde_json::json!([{ "id": milestone.as_str() }]),
                );
            }

            if !fields.is_empty() {
                let body = serde_json::json!({ "fields": fields });
                let path = format!("/rest/api/3/issue/{}", id.as_str());
                self.request_unit(Method::PUT, &path, Some(&body)).await?;
            }

            match (patch.status(), patch.category()) {
                (Some(status), _) => self.transition_status(id.as_str(), status).await?,
                (None, Some(category)) => self.transition_category(id.as_str(), category).await?,
                (None, None) => {}
            }

            self.get(id).await
        })
    }

    fn delete(&self, id: IssueId) -> BoxFuture<'_, IssueResult<()>> {
        Box::pin(async move {
            let path = format!("/rest/api/3/issue/{}", id.as_str());
            self.request_unit(Method::DELETE, &path, None).await
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::issue_filter;

    #[test]
    fn build_jql_bounds_and_orders_when_empty() {
        assert_eq!(
            build_jql(&IssueFilter::default()),
            "created >= \"1970/01/01\" ORDER BY updated DESC"
        );
    }

    #[test]
    fn build_jql_combines_clauses_and_escapes() {
        let filter = issue_filter()
            .project("EN\"G")
            .assignee("acc-1")
            .category(StatusCategory::Started)
            .build();
        assert_eq!(
            build_jql(&filter),
            "project = \"EN\\\"G\" AND assignee = \"acc-1\" AND statusCategory = \"In Progress\" ORDER BY updated DESC"
        );
    }
}
