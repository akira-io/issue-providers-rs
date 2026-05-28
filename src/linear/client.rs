use super::DEFAULT_BASE_URL;

pub struct LinearClientBuilder {
    token: String,
    base_url: String,
}

impl LinearClientBuilder {
    pub(crate) fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
            base_url: DEFAULT_BASE_URL.to_string(),
        }
    }

    #[must_use]
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    pub fn build(self) -> LinearClient {
        LinearClient {
            http: reqwest::Client::new(),
            token: self.token,
            base_url: self.base_url,
        }
    }
}

#[derive(Clone)]
pub struct LinearClient {
    pub(crate) http: reqwest::Client,
    pub(crate) token: String,
    pub(crate) base_url: String,
}
