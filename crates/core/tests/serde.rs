use issue_provider_core::{Issue, Project, ProjectId, StatusCategory, issue};

#[test]
fn issue_round_trips_through_json() -> Result<(), serde_json::Error> {
    let original = issue()
        .id("ISS-1")
        .title("Wire up sync")
        .status("in_progress")
        .category(StatusCategory::Started)
        .project("PRJ-1")
        .milestone("MIL-1")
        .assignee("USR-1")
        .priority(2)
        .updated_at("2026-05-25T00:00:00Z")
        .build();

    let json = serde_json::to_string(&original)?;
    let decoded: Issue = serde_json::from_str(&json)?;

    assert_eq!(decoded, original);
    Ok(())
}

#[test]
fn status_category_round_trips_every_variant() -> Result<(), serde_json::Error> {
    let variants = [
        StatusCategory::Backlog,
        StatusCategory::Unstarted,
        StatusCategory::Started,
        StatusCategory::Completed,
        StatusCategory::Canceled,
    ];

    for variant in variants {
        let json = serde_json::to_string(&variant)?;
        let decoded: StatusCategory = serde_json::from_str(&json)?;
        assert_eq!(decoded, variant);
    }
    Ok(())
}

#[test]
fn named_entity_round_trips() -> Result<(), serde_json::Error> {
    let project = Project::make(ProjectId::make("PRJ-9"), "Platform");
    let json = serde_json::to_string(&project)?;
    let decoded: Project = serde_json::from_str(&json)?;

    assert_eq!(decoded, project);
    Ok(())
}
