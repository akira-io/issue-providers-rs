use issue_provider_core::{ErrorKind, IssueResult, StatusCategory, error};
use reqwest::Method;
use serde::Deserialize;

use crate::client::JiraClient;
use crate::model::StatusCategoryNode;

#[derive(Deserialize)]
struct TransitionNode {
    id: String,
    name: String,
    to: Option<TransitionTarget>,
}

#[derive(Deserialize)]
struct TransitionTarget {
    name: String,
    #[serde(rename = "statusCategory")]
    status_category: Option<StatusCategoryNode>,
}

pub(crate) fn category_status_key(category: StatusCategory) -> &'static str {
    match category {
        StatusCategory::Backlog | StatusCategory::Unstarted => "new",
        StatusCategory::Started => "indeterminate",
        StatusCategory::Completed | StatusCategory::Canceled => "done",
    }
}

impl JiraClient {
    async fn transitions(&self, key: &str) -> IssueResult<Vec<TransitionNode>> {
        #[derive(Deserialize)]
        struct Response {
            #[serde(default)]
            transitions: Vec<TransitionNode>,
        }

        let path = format!("/rest/api/3/issue/{key}/transitions");
        let response: Response = self.request(Method::GET, &path, None).await?;
        Ok(response.transitions)
    }

    async fn apply_transition(&self, key: &str, transition_id: &str) -> IssueResult<()> {
        let path = format!("/rest/api/3/issue/{key}/transitions");
        let body = serde_json::json!({ "transition": { "id": transition_id } });
        self.request_unit(Method::POST, &path, Some(&body)).await
    }

    pub(crate) async fn transition_status(&self, key: &str, status: &str) -> IssueResult<()> {
        let wanted = status.to_lowercase();
        let target = self.transitions(key).await?.into_iter().find(|transition| {
            transition
                .to
                .as_ref()
                .map(|to| to.name.to_lowercase() == wanted)
                .unwrap_or(false)
                || transition.name.to_lowercase() == wanted
        });

        match target {
            Some(transition) => self.apply_transition(key, &transition.id).await,
            None => Err(error().of(
                ErrorKind::Provider,
                format!("jira issue has no transition to status {status}"),
            )),
        }
    }

    pub(crate) async fn transition_category(
        &self,
        key: &str,
        category: StatusCategory,
    ) -> IssueResult<()> {
        let wanted = category_status_key(category);
        let target = self.transitions(key).await?.into_iter().find(|transition| {
            transition
                .to
                .as_ref()
                .and_then(|to| to.status_category.as_ref())
                .map(|status_category| status_category.key == wanted)
                .unwrap_or(false)
        });

        match target {
            Some(transition) => self.apply_transition(key, &transition.id).await,
            None => Err(error().of(
                ErrorKind::Provider,
                format!("jira issue has no transition to category {wanted}"),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn category_status_keys_map_to_jira() {
        assert_eq!(category_status_key(StatusCategory::Backlog), "new");
        assert_eq!(
            category_status_key(StatusCategory::Started),
            "indeterminate"
        );
        assert_eq!(category_status_key(StatusCategory::Canceled), "done");
    }
}
