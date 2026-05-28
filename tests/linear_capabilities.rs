use omnitrack::linear::linear;
use omnitrack::{Capability, Provider};

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
        Capability::Viewer,
        Capability::Comments,
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
        T: omnitrack::Projects
            + omnitrack::Milestones
            + omnitrack::Cycles
            + omnitrack::Teams
            + omnitrack::Users
            + omnitrack::Labels
            + omnitrack::Viewer
            + omnitrack::Comments,
    {
    }

    assert_impls::<omnitrack::linear::LinearClient>();
}
