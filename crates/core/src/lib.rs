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
    Users,
};
pub use errors::{ErrorBuilder, ErrorKind, IssueError, IssueResult, error};
pub use models::{
    Cycle, CycleId, Issue, IssueBuilder, IssueId, Label, LabelId, Milestone, MilestoneId, Missing,
    Project, ProjectId, Set, Team, TeamId, User, UserId, issue,
};
pub use pagination::{Page, PageCursor, PageRequest, PaginationBuilder, pagination};
pub use registry::{
    Provider, ProviderDescriptor, ProviderDescriptorBuilder, ProviderId, ProviderRegistry,
    ProviderRegistryBuilder, provider,
};
