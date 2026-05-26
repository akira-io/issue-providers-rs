use issue_provider_core::{Capability, Provider, ProviderDescriptor, ProviderDescriptorBuilder};

mod client;
mod comments;
mod entities;
mod issues;
mod model;
mod transitions;
mod transport;
mod util;
mod viewer;

pub use client::{JiraClient, JiraClientBuilder};
pub use model::category_from_status_category;

pub const PROVIDER_ID: &str = "jira";
pub const DISPLAY_NAME: &str = "Jira";

#[derive(Clone, Copy, Debug, Default)]
pub struct JiraProvider;

impl JiraProvider {
    pub fn auth(
        self,
        base_url: impl Into<String>,
        email: impl Into<String>,
        token: impl Into<String>,
    ) -> JiraClientBuilder {
        JiraClientBuilder::new(base_url, email, token)
    }
}

impl Provider for JiraProvider {
    fn descriptor(&self) -> ProviderDescriptor {
        ProviderDescriptorBuilder::make(PROVIDER_ID, DISPLAY_NAME)
            .capability(Capability::Issues)
            .capability(Capability::Projects)
            .capability(Capability::Users)
            .capability(Capability::Labels)
            .capability(Capability::Viewer)
            .capability(Capability::Comments)
            .build()
    }
}

pub fn jira() -> JiraProvider {
    JiraProvider
}
