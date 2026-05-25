use issue_provider_core::{IssueId, MilestoneId, ProjectId, UserId, issue};

#[test]
fn issue_builder_constructs_with_required_and_optional_fields() {
    let built = issue()
        .id(IssueId::make("ISS-1"))
        .title("Wire up sync")
        .status("open")
        .project(ProjectId::make("PRJ-1"))
        .milestone(MilestoneId::make("MIL-1"))
        .assignee(UserId::make("USR-1"))
        .priority(2)
        .updated_at("2026-05-25T00:00:00Z")
        .build();

    assert_eq!(built.id().as_str(), "ISS-1");
    assert_eq!(built.title(), "Wire up sync");
    assert_eq!(built.status(), "open");
    assert_eq!(built.project().map(ProjectId::as_str), Some("PRJ-1"));
    assert_eq!(built.milestone().map(MilestoneId::as_str), Some("MIL-1"));
    assert_eq!(built.assignee().map(UserId::as_str), Some("USR-1"));
    assert_eq!(built.priority(), Some(2));
    assert_eq!(built.updated_at(), "2026-05-25T00:00:00Z");
}

#[test]
fn issue_builder_defaults_optionals_to_none() {
    let built = issue()
        .id(IssueId::make("ISS-2"))
        .title("Minimal")
        .status("todo")
        .build();

    assert!(built.project().is_none());
    assert!(built.milestone().is_none());
    assert!(built.assignee().is_none());
    assert_eq!(built.priority(), None);
    assert_eq!(built.updated_at(), "");
}
