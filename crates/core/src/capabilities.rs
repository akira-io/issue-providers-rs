use crate::errors::error;
use crate::models::{
    Cycle, CycleId, Issue, IssueId, Label, LabelId, Milestone, MilestoneId, Project, ProjectId,
    Team, TeamId, User, UserId,
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

capability!(Issues, Issue, IssueId, TransportNotConfiguredIssues);
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
