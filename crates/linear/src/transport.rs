use issue_provider_core::{ErrorKind, IssueError, IssueResult, error};
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::client::LinearClient;

const MAX_ATTEMPTS: u32 = 3;

#[derive(Serialize)]
struct GraphQlRequest<'a> {
    query: &'a str,
    variables: serde_json::Value,
}

#[derive(serde::Deserialize)]
struct GraphQlResponse<T> {
    data: Option<T>,
    errors: Option<Vec<serde_json::Value>>,
}

fn map_status(status: reqwest::StatusCode) -> ErrorKind {
    match status.as_u16() {
        401 | 403 => ErrorKind::Unauthorized,
        404 => ErrorKind::NotFound,
        429 => ErrorKind::RateLimited,
        _ => ErrorKind::Transport,
    }
}

fn format_errors(errors: &[serde_json::Value]) -> String {
    errors
        .iter()
        .map(|e| {
            e.get("message")
                .and_then(|m| m.as_str())
                .unwrap_or("unknown error")
                .to_string()
        })
        .collect::<Vec<_>>()
        .join("; ")
}

impl LinearClient {
    pub(crate) async fn execute<T: DeserializeOwned>(
        &self,
        query: &str,
        variables: serde_json::Value,
    ) -> IssueResult<T> {
        let mut last_transient = String::new();

        for _ in 0..MAX_ATTEMPTS {
            let sent = self
                .http
                .post(&self.base_url)
                .header("Authorization", &self.token)
                .json(&GraphQlRequest {
                    query,
                    variables: variables.clone(),
                })
                .send()
                .await;

            let response = match sent {
                Ok(response) => response,
                Err(err) => {
                    last_transient = format!("request failed: {err}");
                    continue;
                }
            };

            let status = response.status();
            let body = match response.text().await {
                Ok(body) => body,
                Err(err) => {
                    last_transient = format!("response read failed: {err}");
                    continue;
                }
            };

            if !status.is_success() {
                return Err(error().of(map_status(status), format!("{status}: {body}")));
            }

            if body.trim().is_empty() {
                last_transient = "empty response body".to_string();
                continue;
            }

            let parsed: GraphQlResponse<T> = serde_json::from_str(&body)
                .map_err(|err| error().of(ErrorKind::Decode, format!("decode: {err}")))?;

            if let Some(errors) = parsed.errors
                && !errors.is_empty()
            {
                return Err(error().of(ErrorKind::Provider, format_errors(&errors)));
            }

            return match parsed.data {
                Some(data) => Ok(data),
                None => Err(error().of(ErrorKind::Provider, "linear graphql returned no data")),
            };
        }

        Err(transient(&last_transient))
    }
}

fn transient(detail: &str) -> IssueError {
    error().of(
        ErrorKind::Transport,
        format!("linear graphql failed after {MAX_ATTEMPTS} attempts: {detail}"),
    )
}
