use omnitrack::StatusCategory;
use omnitrack::linear::category_from_state_type;

#[test]
fn maps_known_linear_state_types() {
    assert_eq!(
        category_from_state_type("triage"),
        Some(StatusCategory::Backlog)
    );
    assert_eq!(
        category_from_state_type("backlog"),
        Some(StatusCategory::Backlog)
    );
    assert_eq!(
        category_from_state_type("unstarted"),
        Some(StatusCategory::Unstarted)
    );
    assert_eq!(
        category_from_state_type("started"),
        Some(StatusCategory::Started)
    );
    assert_eq!(
        category_from_state_type("completed"),
        Some(StatusCategory::Completed)
    );
    assert_eq!(
        category_from_state_type("canceled"),
        Some(StatusCategory::Canceled)
    );
    assert_eq!(
        category_from_state_type("cancelled"),
        Some(StatusCategory::Canceled)
    );
}

#[test]
fn unknown_state_type_is_none() {
    assert_eq!(category_from_state_type("nonsense"), None);
}

#[test]
fn provider_descriptor_supports_issue_capability() {
    use omnitrack::{Capability, Provider};
    use omnitrack::linear::linear;

    assert!(linear().descriptor().supports(Capability::Issues));
}
