use issue_provider_core::{
    MilestoneId, ProjectId, StatusCategory, TeamId, UserId, issue_draft, issue_patch,
};

#[test]
fn issue_draft_constructs_with_required_and_optional_fields() {
    let draft = issue_draft()
        .team("TEAM-1")
        .title("New issue")
        .status("Todo")
        .category(StatusCategory::Unstarted)
        .project("PRJ-1")
        .milestone("MIL-1")
        .assignee("USR-1")
        .priority(3)
        .build();

    assert_eq!(draft.team().as_str(), "TEAM-1");
    assert_eq!(draft.title(), "New issue");
    assert_eq!(draft.status(), Some("Todo"));
    assert_eq!(draft.category(), Some(StatusCategory::Unstarted));
    assert_eq!(draft.project().map(ProjectId::as_str), Some("PRJ-1"));
    assert_eq!(draft.milestone().map(MilestoneId::as_str), Some("MIL-1"));
    assert_eq!(draft.assignee().map(UserId::as_str), Some("USR-1"));
    assert_eq!(draft.priority(), Some(3));
}

#[test]
fn issue_draft_defaults_optionals_to_none() {
    let draft = issue_draft()
        .team(TeamId::make("TEAM-2"))
        .title("Bare")
        .build();

    assert_eq!(draft.status(), None);
    assert_eq!(draft.category(), None);
    assert!(draft.project().is_none());
    assert!(draft.milestone().is_none());
    assert!(draft.assignee().is_none());
    assert_eq!(draft.priority(), None);
}

#[test]
fn empty_patch_is_empty() {
    assert!(issue_patch().build().is_empty());
}

#[test]
fn patch_captures_set_fields() {
    let patch = issue_patch()
        .title("Renamed")
        .category(StatusCategory::Completed)
        .assignee("USR-9")
        .priority(1)
        .build();

    assert!(!patch.is_empty());
    assert_eq!(patch.title(), Some("Renamed"));
    assert_eq!(patch.category(), Some(StatusCategory::Completed));
    assert_eq!(patch.assignee().map(UserId::as_str), Some("USR-9"));
    assert_eq!(patch.priority(), Some(1));
    assert!(patch.project().is_none());
}
