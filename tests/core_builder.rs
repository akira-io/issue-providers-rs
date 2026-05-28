use omnitrack::{
    IssueId, LabelId, MilestoneId, ProjectId, StatusCategory, TeamId, UserId, comment, issue,
    issue_filter,
};

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

#[test]
fn issue_builder_captures_enriched_fields() {
    let built = issue()
        .id("ISS-9")
        .title("Rich")
        .status("In Progress")
        .identifier("ENG-9")
        .description("body text")
        .url("https://linear.app/x/issue/ENG-9")
        .created_at("2026-05-25T00:00:00Z")
        .updated_at("2026-05-25T01:00:00Z")
        .author("USR-2")
        .team("TEAM-1")
        .labels(["L-1", "L-2"])
        .label("L-3")
        .build();

    assert_eq!(built.identifier(), Some("ENG-9"));
    assert_eq!(built.description(), Some("body text"));
    assert_eq!(built.url(), Some("https://linear.app/x/issue/ENG-9"));
    assert_eq!(built.created_at(), Some("2026-05-25T00:00:00Z"));
    assert_eq!(built.updated_at(), "2026-05-25T01:00:00Z");
    assert_eq!(built.author().map(UserId::as_str), Some("USR-2"));
    assert_eq!(built.team().map(TeamId::as_str), Some("TEAM-1"));
    assert_eq!(
        built
            .labels()
            .iter()
            .map(LabelId::as_str)
            .collect::<Vec<_>>(),
        vec!["L-1", "L-2", "L-3"]
    );
}

#[test]
fn comment_builder_constructs() {
    let c = comment()
        .id("CMT-1")
        .body("looks good")
        .author("USR-1")
        .created_at("2026-05-25T00:00:00Z")
        .build();

    assert_eq!(c.id().as_str(), "CMT-1");
    assert_eq!(c.body(), "looks good");
    assert_eq!(c.author().map(UserId::as_str), Some("USR-1"));
    assert_eq!(c.created_at(), Some("2026-05-25T00:00:00Z"));
}

#[test]
fn issue_filter_builder() {
    let filter = issue_filter()
        .team("TEAM-1")
        .project("PRJ-1")
        .category(StatusCategory::Started)
        .build();

    assert!(!filter.is_empty());
    assert_eq!(filter.team().map(TeamId::as_str), Some("TEAM-1"));
    assert_eq!(filter.project().map(ProjectId::as_str), Some("PRJ-1"));
    assert_eq!(filter.category(), Some(StatusCategory::Started));
    assert!(issue_filter().build().is_empty());
}
