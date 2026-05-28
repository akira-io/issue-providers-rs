use crate::{BoxFuture, IssueResult, User, UserId, Viewer};
use reqwest::Method;
use serde::Deserialize;

use super::client::JiraClient;

impl Viewer for JiraClient {
    fn current_user(&self) -> BoxFuture<'_, IssueResult<User>> {
        Box::pin(async move {
            #[derive(Deserialize)]
            #[serde(rename_all = "camelCase")]
            struct Myself {
                account_id: String,
                display_name: Option<String>,
            }

            let me: Myself = self
                .request(Method::GET, "/rest/api/3/myself", None)
                .await?;
            Ok(User::make(
                UserId::make(me.account_id),
                me.display_name.unwrap_or_default(),
            ))
        })
    }
}
