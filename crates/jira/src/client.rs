pub struct JiraClientBuilder {
    base_url: String,
    email: String,
    token: String,
}

impl JiraClientBuilder {
    pub(crate) fn new(
        base_url: impl Into<String>,
        email: impl Into<String>,
        token: impl Into<String>,
    ) -> Self {
        Self {
            base_url: base_url.into(),
            email: email.into(),
            token: token.into(),
        }
    }

    pub fn build(self) -> JiraClient {
        JiraClient {
            http: reqwest::Client::new(),
            base_url: self.base_url,
            email: self.email,
            token: self.token,
        }
    }
}

#[derive(Clone)]
pub struct JiraClient {
    pub(crate) http: reqwest::Client,
    pub(crate) base_url: String,
    pub(crate) email: String,
    pub(crate) token: String,
}
