use omnitrack::{
    Capability, Provider, ProviderDescriptor, ProviderDescriptorBuilder, provider,
};

struct Dummy(&'static str);

impl Provider for Dummy {
    fn descriptor(&self) -> ProviderDescriptor {
        ProviderDescriptorBuilder::make(self.0, self.0)
            .capability(Capability::Issues)
            .build()
    }
}

#[test]
fn registry_collects_registered_providers() -> omnitrack::IssueResult<()> {
    let registry = provider()
        .register(Dummy("a"))?
        .register(Dummy("b"))?
        .build();
    assert_eq!(registry.descriptors().len(), 2);
    Ok(())
}

#[test]
fn registry_rejects_duplicate_ids() -> omnitrack::IssueResult<()> {
    let result = provider().register(Dummy("a"))?.register(Dummy("a"));
    assert!(result.is_err());
    Ok(())
}

#[test]
fn descriptor_reports_capabilities() {
    let descriptor = ProviderDescriptorBuilder::make("linear", "Linear")
        .capability(Capability::Issues)
        .build();
    assert!(descriptor.supports(Capability::Issues));
    assert!(!descriptor.supports(Capability::Projects));
}
