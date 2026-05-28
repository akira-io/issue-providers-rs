use crate::{Capability, Provider, ProviderDescriptor, ProviderDescriptorBuilder};

mod client;
mod comments;
mod entities;
mod issues;
mod transport;
mod viewer;

pub use client::{LinearClient, LinearClientBuilder};
pub use issues::category_from_state_type;

pub const PROVIDER_ID: &str = "linear";
pub const DISPLAY_NAME: &str = "Linear";
pub const DEFAULT_BASE_URL: &str = "https://api.linear.app/graphql";

#[derive(Clone, Copy, Debug, Default)]
pub struct LinearProvider;

impl LinearProvider {
    pub fn token(self, token: impl Into<String>) -> LinearClientBuilder {
        LinearClientBuilder::new(token)
    }
}

impl Provider for LinearProvider {
    fn descriptor(&self) -> ProviderDescriptor {
        ProviderDescriptorBuilder::make(PROVIDER_ID, DISPLAY_NAME)
            .capability(Capability::Issues)
            .capability(Capability::Projects)
            .capability(Capability::Milestones)
            .capability(Capability::Cycles)
            .capability(Capability::Teams)
            .capability(Capability::Users)
            .capability(Capability::Labels)
            .capability(Capability::Viewer)
            .capability(Capability::Comments)
            .build()
    }
}

pub fn linear() -> LinearProvider {
    LinearProvider
}
