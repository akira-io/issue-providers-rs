use std::future::Future;
use std::pin::Pin;

mod capabilities;
mod errors;
mod models;
mod pagination;
mod registry;

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

pub use capabilities::{
    Capability, Cycles, Issues, Labels, Milestones, Projects, Teams, TransportNotConfiguredCycles,
    TransportNotConfiguredIssues, TransportNotConfiguredLabels, TransportNotConfiguredMilestones,
    TransportNotConfiguredProjects, TransportNotConfiguredTeams, TransportNotConfiguredUsers,
    TransportNotConfiguredViewer, Users, Viewer,
};
pub use errors::{ErrorBuilder, ErrorKind, IssueError, IssueResult, error};
pub use models::{
    Cycle, CycleId, Issue, IssueBuilder, IssueDraft, IssueDraftBuilder, IssueId, IssuePatch,
    IssuePatchBuilder, Label, LabelId, Milestone, MilestoneId, Missing, Project, ProjectId, Set,
    StatusCategory, Team, TeamId, User, UserId, issue, issue_draft, issue_patch,
};
pub use pagination::{Page, PageCursor, PageRequest, PaginationBuilder, pagination};
pub use registry::{
    Provider, ProviderDescriptor, ProviderDescriptorBuilder, ProviderId, ProviderRegistry,
    ProviderRegistryBuilder, provider,
};
