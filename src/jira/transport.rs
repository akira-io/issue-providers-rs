use crate::{ErrorKind, IssueError, IssueResult, error};
use reqwest::Method;
use serde::de::DeserializeOwned;

use super::client::{JiraAuth, JiraClient};

const MAX_ATTEMPTS: u32 = 3;

fn map_status(status: reqwest::StatusCode) -> ErrorKind {
    match status.as_u16() {
        401 | 403 => ErrorKind::Unauthorized,
        404 => ErrorKind::NotFound,
        429 => ErrorKind::RateLimited,
        _ => ErrorKind::Transport,
    }
}

impl JiraClient {
    async fn send(
        &self,
        method: Method,
        path: &str,
        body: Option<&serde_json::Value>,
    ) -> IssueResult<String> {
        let url = format!("{}{}", self.base_url.trim_end_matches('/'), path);
        let mut last_transient = String::new();

        for _ in 0..MAX_ATTEMPTS {
            let mut request = self
                .http
                .request(method.clone(), &url)
                .header("Accept", "application/json");
            request = match &self.auth {
                JiraAuth::Basic { email, token } => request.basic_auth(email, Some(token)),
                JiraAuth::Bearer { token } => request.bearer_auth(token),
            };
            if let Some(body) = body {
                request = request.json(body);
            }

            let sent = request.send().await;
            let response = match sent {
                Ok(response) => response,
                Err(err) => {
                    last_transient = format!("request failed: {err}");
                    continue;
                }
            };

            let status = response.status();
            let text = match response.text().await {
                Ok(text) => text,
                Err(err) => {
                    last_transient = format!("response read failed: {err}");
                    continue;
                }
            };

            if !status.is_success() {
                return Err(error().of(map_status(status), format!("{status}: {text}")));
            }

            return Ok(text);
        }

        Err(transient(&last_transient))
    }

    pub(crate) async fn request<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        body: Option<&serde_json::Value>,
    ) -> IssueResult<T> {
        let text = self.send(method, path, body).await?;
        serde_json::from_str(&text)
            .map_err(|err| error().of(ErrorKind::Decode, format!("decode: {err}")))
    }

    pub(crate) async fn request_unit(
        &self,
        method: Method,
        path: &str,
        body: Option<&serde_json::Value>,
    ) -> IssueResult<()> {
        self.send(method, path, body).await.map(|_| ())
    }
}

fn transient(detail: &str) -> IssueError {
    error().of(
        ErrorKind::Transport,
        format!("jira request failed after {MAX_ATTEMPTS} attempts: {detail}"),
    )
}
