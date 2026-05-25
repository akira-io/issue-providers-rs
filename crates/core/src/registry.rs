use std::collections::BTreeSet;

use crate::IssueResult;
use crate::capabilities::Capability;
use crate::errors::{ErrorKind, error};

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ProviderId(String);

impl ProviderId {
    pub fn make(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug)]
pub struct ProviderDescriptor {
    id: ProviderId,
    display_name: String,
    capabilities: BTreeSet<Capability>,
}

impl ProviderDescriptor {
    pub fn id(&self) -> &ProviderId {
        &self.id
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub fn capabilities(&self) -> &BTreeSet<Capability> {
        &self.capabilities
    }

    pub fn supports(&self, capability: Capability) -> bool {
        self.capabilities.contains(&capability)
    }
}

pub struct ProviderDescriptorBuilder {
    id: ProviderId,
    display_name: String,
    capabilities: BTreeSet<Capability>,
}

impl ProviderDescriptorBuilder {
    pub fn make(id: impl Into<String>, display_name: impl Into<String>) -> Self {
        Self {
            id: ProviderId::make(id),
            display_name: display_name.into(),
            capabilities: BTreeSet::new(),
        }
    }

    #[must_use]
    pub fn capability(mut self, capability: Capability) -> Self {
        self.capabilities.insert(capability);
        self
    }

    pub fn build(self) -> ProviderDescriptor {
        ProviderDescriptor {
            id: self.id,
            display_name: self.display_name,
            capabilities: self.capabilities,
        }
    }
}

pub trait Provider: Send + Sync {
    fn descriptor(&self) -> ProviderDescriptor;
}

#[derive(Default)]
pub struct ProviderRegistry {
    providers: Vec<Box<dyn Provider>>,
}

impl ProviderRegistry {
    pub fn descriptors(&self) -> Vec<ProviderDescriptor> {
        self.providers
            .iter()
            .map(|provider| provider.descriptor())
            .collect()
    }
}

#[derive(Default)]
pub struct ProviderRegistryBuilder {
    providers: Vec<Box<dyn Provider>>,
    ids: BTreeSet<ProviderId>,
}

impl ProviderRegistryBuilder {
    pub fn register(mut self, provider: impl Provider + 'static) -> IssueResult<Self> {
        let id = provider.descriptor().id().clone();
        if !self.ids.insert(id.clone()) {
            return Err(error().of(
                ErrorKind::Provider,
                format!("provider already registered: {}", id.as_str()),
            ));
        }
        self.providers.push(Box::new(provider));
        Ok(self)
    }

    pub fn build(self) -> ProviderRegistry {
        ProviderRegistry {
            providers: self.providers,
        }
    }
}

pub fn provider() -> ProviderRegistryBuilder {
    ProviderRegistryBuilder::default()
}
