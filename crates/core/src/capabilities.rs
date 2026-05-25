use crate::errors::error;
use crate::models::{
    Cycle, CycleId, Issue, IssueDraft, IssueId, IssuePatch, Label, LabelId, Milestone, MilestoneId,
    Project, ProjectId, StatusCategory, Team, TeamId, User, UserId, issue_patch,
};
use crate::pagination::{Page, PageRequest};
use crate::{BoxFuture, IssueResult};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Capability {
    Issues,
    Projects,
    Milestones,
    Cycles,
    Teams,
    Users,
    Labels,
}

macro_rules! capability {
    ($trait_name:ident, $entity:ty, $id:ty, $not_configured:ident) => {
        pub trait $trait_name: Send + Sync {
            fn get(&self, id: $id) -> BoxFuture<'_, IssueResult<$entity>>;

            fn list(&self, page: Option<PageRequest>) -> BoxFuture<'_, IssueResult<Page<$entity>>>;
        }

        #[derive(Clone, Copy, Debug, Default)]
        pub struct $not_configured;

        impl $trait_name for $not_configured {
            fn get(&self, _id: $id) -> BoxFuture<'_, IssueResult<$entity>> {
                Box::pin(async { Err(error().transport_not_configured()) })
            }

            fn list(
                &self,
                _page: Option<PageRequest>,
            ) -> BoxFuture<'_, IssueResult<Page<$entity>>> {
                Box::pin(async { Err(error().transport_not_configured()) })
            }
        }
    };
}

pub trait Issues: Send + Sync {
    fn get(&self, id: IssueId) -> BoxFuture<'_, IssueResult<Issue>>;

    fn list(&self, page: Option<PageRequest>) -> BoxFuture<'_, IssueResult<Page<Issue>>>;

    fn create(&self, draft: IssueDraft) -> BoxFuture<'_, IssueResult<Issue>>;

    fn update(&self, id: IssueId, patch: IssuePatch) -> BoxFuture<'_, IssueResult<Issue>>;

    fn delete(&self, id: IssueId) -> BoxFuture<'_, IssueResult<()>>;

    fn close(&self, id: IssueId) -> BoxFuture<'_, IssueResult<Issue>> {
        self.update(
            id,
            issue_patch().category(StatusCategory::Completed).build(),
        )
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TransportNotConfiguredIssues;

impl Issues for TransportNotConfiguredIssues {
    fn get(&self, _id: IssueId) -> BoxFuture<'_, IssueResult<Issue>> {
        Box::pin(async { Err(error().transport_not_configured()) })
    }

    fn list(&self, _page: Option<PageRequest>) -> BoxFuture<'_, IssueResult<Page<Issue>>> {
        Box::pin(async { Err(error().transport_not_configured()) })
    }

    fn create(&self, _draft: IssueDraft) -> BoxFuture<'_, IssueResult<Issue>> {
        Box::pin(async { Err(error().transport_not_configured()) })
    }

    fn update(&self, _id: IssueId, _patch: IssuePatch) -> BoxFuture<'_, IssueResult<Issue>> {
        Box::pin(async { Err(error().transport_not_configured()) })
    }

    fn delete(&self, _id: IssueId) -> BoxFuture<'_, IssueResult<()>> {
        Box::pin(async { Err(error().transport_not_configured()) })
    }
}

capability!(Projects, Project, ProjectId, TransportNotConfiguredProjects);
capability!(
    Milestones,
    Milestone,
    MilestoneId,
    TransportNotConfiguredMilestones
);
capability!(Cycles, Cycle, CycleId, TransportNotConfiguredCycles);
capability!(Teams, Team, TeamId, TransportNotConfiguredTeams);
capability!(Users, User, UserId, TransportNotConfiguredUsers);
capability!(Labels, Label, LabelId, TransportNotConfiguredLabels);
