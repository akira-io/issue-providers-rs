use crate::{ErrorKind, IssueResult, error};
use serde::Deserialize;

const ACCESSIBLE_RESOURCES_URL: &str = "https://api.atlassian.com/oauth/token/accessible-resources";

#[derive(Clone, Debug, Deserialize)]
pub struct JiraSite {
    #[serde(rename = "id")]
    pub cloud_id: String,
    pub url: String,
    #[serde(default)]
    pub name: String,
}

/// List the Jira sites an OAuth 2.0 (3LO) access token can reach, each with its
/// `cloudId`, used to build the `api.atlassian.com/ex/jira/{cloudId}` base.
pub async fn accessible_resources(access_token: &str) -> IssueResult<Vec<JiraSite>> {
    let response = reqwest::Client::new()
        .get(ACCESSIBLE_RESOURCES_URL)
        .bearer_auth(access_token)
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(|err| {
            error().of(
                ErrorKind::Transport,
                format!("accessible-resources request failed: {err}"),
            )
        })?;

    let status = response.status();
    let text = response.text().await.map_err(|err| {
        error().of(
            ErrorKind::Transport,
            format!("accessible-resources read failed: {err}"),
        )
    })?;

    if !status.is_success() {
        let kind = match status.as_u16() {
            401 | 403 => ErrorKind::Unauthorized,
            429 => ErrorKind::RateLimited,
            _ => ErrorKind::Transport,
        };

        return Err(error().of(kind, format!("{status}: {text}")));
    }

    serde_json::from_str(&text)
        .map_err(|err| error().of(ErrorKind::Decode, format!("decode: {err}")))
}
