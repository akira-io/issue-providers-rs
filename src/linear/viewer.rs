use crate::{BoxFuture, ErrorKind, IssueResult, User, UserId, Viewer, error};
use serde::Deserialize;

use super::client::LinearClient;

impl Viewer for LinearClient {
    fn current_user(&self) -> BoxFuture<'_, IssueResult<User>> {
        Box::pin(async move {
            #[derive(Deserialize)]
            struct Data {
                viewer: Option<ViewerNode>,
            }
            #[derive(Deserialize)]
            struct ViewerNode {
                id: String,
                name: Option<String>,
            }

            let query = "query { viewer { id name } }";
            let data: Data = self.execute(query, serde_json::json!({})).await?;

            data.viewer
                .map(|node| User::make(UserId::make(node.id), node.name.unwrap_or_default()))
                .ok_or_else(|| error().of(ErrorKind::Provider, "linear viewer returned no user"))
        })
    }
}
