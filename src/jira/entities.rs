use crate::{
    BoxFuture, IssueResult, Label, LabelId, Labels, Page, PageRequest, Project, ProjectId,
    Projects, User, UserId, Users,
};
use reqwest::Method;
use serde::Deserialize;

use super::client::JiraClient;
use super::util::{max_results, offset_cursor, start_at};

#[derive(Deserialize)]
struct ProjectNode {
    key: String,
    name: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct UserNode {
    account_id: String,
    display_name: Option<String>,
}

impl Projects for JiraClient {
    fn get(&self, id: ProjectId) -> BoxFuture<'_, IssueResult<Project>> {
        Box::pin(async move {
            let path = format!("/rest/api/3/project/{}", id.as_str());
            let node: ProjectNode = self.request(Method::GET, &path, None).await?;
            Ok(Project::make(
                ProjectId::make(node.key),
                node.name.unwrap_or_default(),
            ))
        })
    }

    fn list(&self, page: Option<PageRequest>) -> BoxFuture<'_, IssueResult<Page<Project>>> {
        Box::pin(async move {
            #[derive(Deserialize)]
            struct Search {
                #[serde(default)]
                values: Vec<ProjectNode>,
                #[serde(rename = "isLast")]
                is_last: Option<bool>,
            }

            let start = start_at(&page);
            let max = max_results(&page);
            let path = format!("/rest/api/3/project/search?startAt={start}&maxResults={max}");
            let data: Search = self.request(Method::GET, &path, None).await?;

            let count = data.values.len() as u32;
            let items = data
                .values
                .into_iter()
                .map(|node| Project::make(ProjectId::make(node.key), node.name.unwrap_or_default()))
                .collect();
            let next = offset_cursor(data.is_last, start, count, max);

            Ok(Page::make(items, next))
        })
    }
}

impl Users for JiraClient {
    fn get(&self, id: UserId) -> BoxFuture<'_, IssueResult<User>> {
        Box::pin(async move {
            let path = format!("/rest/api/3/user?accountId={}", id.as_str());
            let node: UserNode = self.request(Method::GET, &path, None).await?;
            Ok(User::make(
                UserId::make(node.account_id),
                node.display_name.unwrap_or_default(),
            ))
        })
    }

    fn list(&self, page: Option<PageRequest>) -> BoxFuture<'_, IssueResult<Page<User>>> {
        Box::pin(async move {
            let start = start_at(&page);
            let max = max_results(&page);
            let path = format!("/rest/api/3/users/search?startAt={start}&maxResults={max}");
            let nodes: Vec<UserNode> = self.request(Method::GET, &path, None).await?;

            let count = nodes.len() as u32;
            let items = nodes
                .into_iter()
                .map(|node| {
                    User::make(
                        UserId::make(node.account_id),
                        node.display_name.unwrap_or_default(),
                    )
                })
                .collect();
            let next = offset_cursor(None, start, count, max);

            Ok(Page::make(items, next))
        })
    }
}

impl Labels for JiraClient {
    fn get(&self, id: LabelId) -> BoxFuture<'_, IssueResult<Label>> {
        Box::pin(async move {
            let name = id.as_str().to_string();
            Ok(Label::make(LabelId::make(name.clone()), name))
        })
    }

    fn list(&self, page: Option<PageRequest>) -> BoxFuture<'_, IssueResult<Page<Label>>> {
        Box::pin(async move {
            #[derive(Deserialize)]
            struct LabelsResponse {
                #[serde(default)]
                values: Vec<String>,
                #[serde(rename = "isLast")]
                is_last: Option<bool>,
            }

            let start = start_at(&page);
            let max = max_results(&page);
            let path = format!("/rest/api/3/label?startAt={start}&maxResults={max}");
            let data: LabelsResponse = self.request(Method::GET, &path, None).await?;

            let count = data.values.len() as u32;
            let items = data
                .values
                .into_iter()
                .map(|name| Label::make(LabelId::make(name.clone()), name))
                .collect();
            let next = offset_cursor(data.is_last, start, count, max);

            Ok(Page::make(items, next))
        })
    }
}
