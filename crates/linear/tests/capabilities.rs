use issue_provider_core::{Capability, Provider};
use issue_provider_linear::linear;

#[test]
fn descriptor_supports_all_named_capabilities() {
    let descriptor = linear().descriptor();
    for capability in [
        Capability::Issues,
        Capability::Projects,
        Capability::Milestones,
        Capability::Cycles,
        Capability::Teams,
        Capability::Users,
        Capability::Labels,
    ] {
        assert!(
            descriptor.supports(capability),
            "missing capability: {capability:?}"
        );
    }
}

#[test]
fn client_implements_named_capability_traits() {
    fn assert_impls<T>()
    where
        T: issue_provider_core::Projects
            + issue_provider_core::Milestones
            + issue_provider_core::Cycles
            + issue_provider_core::Teams
            + issue_provider_core::Users
            + issue_provider_core::Labels,
    {
    }

    assert_impls::<issue_provider_linear::LinearClient>();
}
