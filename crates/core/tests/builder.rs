use issue_provider_core::{IssueId, MilestoneId, ProjectId, StatusCategory, UserId, issue};

#[test]
fn issue_builder_constructs_with_required_and_optional_fields() {
    let built = issue()
        .id("ISS-1")
        .title("Wire up sync")
        .status("open")
        .category(StatusCategory::Started)
        .project("PRJ-1")
        .milestone("MIL-1")
        .assignee("USR-1")
        .priority(2)
        .updated_at("2026-05-25T00:00:00Z")
        .build();

    assert_eq!(built.id().as_str(), "ISS-1");
    assert_eq!(built.title(), "Wire up sync");
    assert_eq!(built.status(), "open");
    assert_eq!(built.category(), Some(StatusCategory::Started));
    assert_eq!(built.project().map(ProjectId::as_str), Some("PRJ-1"));
    assert_eq!(built.milestone().map(MilestoneId::as_str), Some("MIL-1"));
    assert_eq!(built.assignee().map(UserId::as_str), Some("USR-1"));
    assert_eq!(built.priority(), Some(2));
    assert_eq!(built.updated_at(), "2026-05-25T00:00:00Z");
}

#[test]
fn issue_builder_defaults_optionals_to_none() {
    let built = issue().id("ISS-2").title("Minimal").status("todo").build();

    assert_eq!(built.category(), None);
    assert!(built.project().is_none());
    assert!(built.milestone().is_none());
    assert!(built.assignee().is_none());
    assert_eq!(built.priority(), None);
    assert_eq!(built.updated_at(), "");
}

#[test]
fn builder_accepts_raw_strings_or_newtypes() {
    let from_strings = issue()
        .id("ISS-1")
        .title("t")
        .status("s")
        .project("PRJ-1")
        .build();
    let from_newtypes = issue()
        .id(IssueId::make("ISS-1"))
        .title("t")
        .status("s")
        .project(ProjectId::make("PRJ-1"))
        .build();

    assert_eq!(from_strings.id().as_str(), from_newtypes.id().as_str());
    assert_eq!(
        from_strings.project().map(ProjectId::as_str),
        from_newtypes.project().map(ProjectId::as_str)
    );
}
