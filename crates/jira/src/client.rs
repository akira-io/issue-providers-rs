const API_GATEWAY: &str = "https://api.atlassian.com/ex/jira";

#[derive(Clone)]
pub(crate) enum JiraAuth {
    Basic { email: String, token: String },
    Bearer { token: String },
}

pub struct JiraClientBuilder {
    base_url: String,
    auth: JiraAuth,
}

impl JiraClientBuilder {
    pub(crate) fn basic(
        base_url: impl Into<String>,
        email: impl Into<String>,
        token: impl Into<String>,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            auth: JiraAuth::Basic {
                email: email.into(),
                token: token.into(),
            },
        }
    }

    pub(crate) fn bearer(cloud_id: impl Into<String>, access_token: impl Into<String>) -> Self {
        Self {
            base_url: format!("{API_GATEWAY}/{}", cloud_id.into()),
            auth: JiraAuth::Bearer {
                token: access_token.into(),
            },
        }
    }

    pub fn build(self) -> JiraClient {
        JiraClient {
            http: reqwest::Client::new(),
            base_url: self.base_url,
            auth: self.auth,
        }
    }
}

#[derive(Clone)]
pub struct JiraClient {
    pub(crate) http: reqwest::Client,
    pub(crate) base_url: String,
    pub(crate) auth: JiraAuth,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bearer_targets_the_oauth_gateway() {
        let client = JiraClientBuilder::bearer("cloud-123", "tok").build();
        assert_eq!(
            client.base_url,
            "https://api.atlassian.com/ex/jira/cloud-123"
        );
        assert!(matches!(client.auth, JiraAuth::Bearer { .. }));
    }

    #[test]
    fn basic_keeps_the_site_base() {
        let client = JiraClientBuilder::basic("https://x.atlassian.net", "me@x.com", "tok").build();
        assert_eq!(client.base_url, "https://x.atlassian.net");
        assert!(matches!(client.auth, JiraAuth::Basic { .. }));
    }
}
