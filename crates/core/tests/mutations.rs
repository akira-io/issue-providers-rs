use issue_provider_core::{
    BoxFuture, Issue, IssueDraft, IssueId, IssuePatch, IssueResult, Issues, MilestoneId, Page,
    PageRequest, ProjectId, StatusCategory, TeamId, UserId, issue, issue_draft, issue_patch,
};

struct EchoClient;

impl Issues for EchoClient {
    fn get(&self, id: IssueId) -> BoxFuture<'_, IssueResult<Issue>> {
        Box::pin(async move { Ok(issue().id(id).title("got").status("open").build()) })
    }

    fn list(&self, _page: Option<PageRequest>) -> BoxFuture<'_, IssueResult<Page<Issue>>> {
        Box::pin(async { Ok(Page::make(Vec::new(), None)) })
    }

    fn create(&self, draft: IssueDraft) -> BoxFuture<'_, IssueResult<Issue>> {
        Box::pin(async move {
            Ok(issue()
                .id("NEW-1")
                .title(draft.title().to_string())
                .status("open")
                .build())
        })
    }

    fn update(&self, id: IssueId, patch: IssuePatch) -> BoxFuture<'_, IssueResult<Issue>> {
        Box::pin(async move {
            Ok(issue()
                .id(id)
                .title(patch.title().unwrap_or("unchanged").to_string())
                .status("open")
                .build())
        })
    }

    fn delete(&self, _id: IssueId) -> BoxFuture<'_, IssueResult<()>> {
        Box::pin(async { Ok(()) })
    }
}

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

#[test]
fn draft_create_terminal_executes_against_client() -> IssueResult<()> {
    let client = EchoClient;
    let created =
        futures::executor::block_on(issue_draft().team("TEAM-1").title("Wired").create(&client))?;

    assert_eq!(created.id().as_str(), "NEW-1");
    assert_eq!(created.title(), "Wired");
    Ok(())
}

#[test]
fn patch_update_terminal_executes_against_client() -> IssueResult<()> {
    let client = EchoClient;
    let updated =
        futures::executor::block_on(issue_patch().title("Renamed").update(&client, "ISS-7"))?;

    assert_eq!(updated.id().as_str(), "ISS-7");
    assert_eq!(updated.title(), "Renamed");
    Ok(())
}
