use crate::{Issue, StatusCategory, issue};
use serde::Deserialize;

use super::util::adf_text;

pub(crate) const ISSUE_FIELDS: &str = "summary,status,project,assignee,reporter,labels,priority,created,updated,fixVersions,description";

#[derive(Deserialize)]
pub(crate) struct IssueNode {
    key: String,
    fields: IssueFields,
}

#[derive(Deserialize)]
struct IssueFields {
    summary: Option<String>,
    status: Option<StatusNode>,
    project: Option<KeyNode>,
    assignee: Option<AccountNode>,
    reporter: Option<AccountNode>,
    #[serde(default)]
    labels: Vec<String>,
    priority: Option<PriorityNode>,
    created: Option<String>,
    updated: Option<String>,
    #[serde(rename = "fixVersions", default)]
    fix_versions: Vec<IdNode>,
    description: Option<serde_json::Value>,
}

#[derive(Deserialize)]
struct StatusNode {
    name: String,
    #[serde(rename = "statusCategory")]
    status_category: Option<StatusCategoryNode>,
}

#[derive(Deserialize)]
pub(crate) struct StatusCategoryNode {
    pub(crate) key: String,
}

#[derive(Deserialize)]
struct KeyNode {
    key: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AccountNode {
    account_id: Option<String>,
}

#[derive(Deserialize)]
struct PriorityNode {
    id: Option<String>,
}

#[derive(Deserialize)]
struct IdNode {
    id: String,
}

pub fn category_from_status_category(key: &str) -> Option<StatusCategory> {
    Some(match key {
        "new" => StatusCategory::Unstarted,
        "indeterminate" => StatusCategory::Started,
        "done" => StatusCategory::Completed,
        _ => return None,
    })
}

pub(crate) fn map_issue(node: IssueNode, base_url: &str) -> Issue {
    let fields = node.fields;
    let status = fields
        .status
        .as_ref()
        .map(|status| status.name.clone())
        .unwrap_or_default();

    let mut builder = issue()
        .id(node.key.clone())
        .title(fields.summary.unwrap_or_default())
        .status(status)
        .identifier(node.key.clone())
        .url(format!(
            "{}/browse/{}",
            base_url.trim_end_matches('/'),
            node.key
        ));

    if let Some(category) = fields
        .status
        .as_ref()
        .and_then(|status| status.status_category.as_ref())
        .and_then(|category| category_from_status_category(&category.key))
    {
        builder = builder.category(category);
    }
    if let Some(project) = fields.project {
        builder = builder.project(project.key);
    }
    if let Some(assignee) = fields.assignee.and_then(|account| account.account_id) {
        builder = builder.assignee(assignee);
    }
    if let Some(reporter) = fields.reporter.and_then(|account| account.account_id) {
        builder = builder.author(reporter);
    }
    if !fields.labels.is_empty() {
        builder = builder.labels(fields.labels);
    }
    if let Some(priority) = fields
        .priority
        .and_then(|priority| priority.id)
        .and_then(|id| id.parse::<u8>().ok())
    {
        builder = builder.priority(priority);
    }
    if let Some(version) = fields.fix_versions.into_iter().next() {
        builder = builder.milestone(version.id);
    }
    if let Some(created) = fields.created {
        builder = builder.created_at(created);
    }
    if let Some(updated) = fields.updated {
        builder = builder.updated_at(updated);
    }
    if let Some(description) = fields
        .description
        .as_ref()
        .map(adf_text)
        .filter(|text| !text.is_empty())
    {
        builder = builder.description(description);
    }

    builder.build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_category_round_trips() {
        assert_eq!(
            category_from_status_category("indeterminate"),
            Some(StatusCategory::Started)
        );
        assert_eq!(
            category_from_status_category("done"),
            Some(StatusCategory::Completed)
        );
        assert_eq!(category_from_status_category("unknown"), None);
    }
}
