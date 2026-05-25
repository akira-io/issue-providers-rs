use serde::{Deserialize, Serialize};

macro_rules! id_newtype {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
        pub struct $name(String);

        impl $name {
            pub fn make(value: impl Into<String>) -> Self {
                Self(value.into())
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                Self(value.to_string())
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self(value)
            }
        }
    };
}

id_newtype!(IssueId);
id_newtype!(ProjectId);
id_newtype!(MilestoneId);
id_newtype!(CycleId);
id_newtype!(TeamId);
id_newtype!(UserId);
id_newtype!(LabelId);
id_newtype!(CommentId);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum StatusCategory {
    Backlog,
    Unstarted,
    Started,
    Completed,
    Canceled,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
struct IssueMeta {
    category: Option<StatusCategory>,
    project: Option<ProjectId>,
    milestone: Option<MilestoneId>,
    assignee: Option<UserId>,
    author: Option<UserId>,
    team: Option<TeamId>,
    labels: Vec<LabelId>,
    priority: Option<u8>,
    identifier: Option<String>,
    description: Option<String>,
    url: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Issue {
    id: IssueId,
    title: String,
    status: String,
    #[serde(flatten)]
    meta: IssueMeta,
}

impl Issue {
    pub fn id(&self) -> &IssueId {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn status(&self) -> &str {
        &self.status
    }

    pub fn category(&self) -> Option<StatusCategory> {
        self.meta.category
    }

    pub fn project(&self) -> Option<&ProjectId> {
        self.meta.project.as_ref()
    }

    pub fn milestone(&self) -> Option<&MilestoneId> {
        self.meta.milestone.as_ref()
    }

    pub fn assignee(&self) -> Option<&UserId> {
        self.meta.assignee.as_ref()
    }

    pub fn author(&self) -> Option<&UserId> {
        self.meta.author.as_ref()
    }

    pub fn team(&self) -> Option<&TeamId> {
        self.meta.team.as_ref()
    }

    pub fn labels(&self) -> &[LabelId] {
        &self.meta.labels
    }

    pub fn priority(&self) -> Option<u8> {
        self.meta.priority
    }

    pub fn identifier(&self) -> Option<&str> {
        self.meta.identifier.as_deref()
    }

    pub fn description(&self) -> Option<&str> {
        self.meta.description.as_deref()
    }

    pub fn url(&self) -> Option<&str> {
        self.meta.url.as_deref()
    }

    pub fn created_at(&self) -> Option<&str> {
        self.meta.created_at.as_deref()
    }

    pub fn updated_at(&self) -> &str {
        self.meta.updated_at.as_deref().unwrap_or_default()
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Missing;

#[derive(Clone, Debug)]
pub struct Set<T>(T);

pub struct IssueBuilder<I, T, S> {
    id: I,
    title: T,
    status: S,
    meta: IssueMeta,
}

pub fn issue() -> IssueBuilder<Missing, Missing, Missing> {
    IssueBuilder {
        id: Missing,
        title: Missing,
        status: Missing,
        meta: IssueMeta::default(),
    }
}

impl<T, S> IssueBuilder<Missing, T, S> {
    pub fn id(self, id: impl Into<IssueId>) -> IssueBuilder<Set<IssueId>, T, S> {
        IssueBuilder {
            id: Set(id.into()),
            title: self.title,
            status: self.status,
            meta: self.meta,
        }
    }
}

impl<I, S> IssueBuilder<I, Missing, S> {
    pub fn title(self, title: impl Into<String>) -> IssueBuilder<I, Set<String>, S> {
        IssueBuilder {
            id: self.id,
            title: Set(title.into()),
            status: self.status,
            meta: self.meta,
        }
    }
}

impl<I, T> IssueBuilder<I, T, Missing> {
    pub fn status(self, status: impl Into<String>) -> IssueBuilder<I, T, Set<String>> {
        IssueBuilder {
            id: self.id,
            title: self.title,
            status: Set(status.into()),
            meta: self.meta,
        }
    }
}

impl<I, T, S> IssueBuilder<I, T, S> {
    #[must_use]
    pub fn category(mut self, category: StatusCategory) -> Self {
        self.meta.category = Some(category);
        self
    }

    #[must_use]
    pub fn project(mut self, project: impl Into<ProjectId>) -> Self {
        self.meta.project = Some(project.into());
        self
    }

    #[must_use]
    pub fn milestone(mut self, milestone: impl Into<MilestoneId>) -> Self {
        self.meta.milestone = Some(milestone.into());
        self
    }

    #[must_use]
    pub fn assignee(mut self, assignee: impl Into<UserId>) -> Self {
        self.meta.assignee = Some(assignee.into());
        self
    }

    #[must_use]
    pub fn author(mut self, author: impl Into<UserId>) -> Self {
        self.meta.author = Some(author.into());
        self
    }

    #[must_use]
    pub fn team(mut self, team: impl Into<TeamId>) -> Self {
        self.meta.team = Some(team.into());
        self
    }

    #[must_use]
    pub fn labels<I2, L>(mut self, labels: I2) -> Self
    where
        I2: IntoIterator<Item = L>,
        L: Into<LabelId>,
    {
        self.meta.labels = labels.into_iter().map(Into::into).collect();
        self
    }

    #[must_use]
    pub fn label(mut self, label: impl Into<LabelId>) -> Self {
        self.meta.labels.push(label.into());
        self
    }

    #[must_use]
    pub fn priority(mut self, priority: u8) -> Self {
        self.meta.priority = Some(priority);
        self
    }

    #[must_use]
    pub fn identifier(mut self, identifier: impl Into<String>) -> Self {
        self.meta.identifier = Some(identifier.into());
        self
    }

    #[must_use]
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.meta.description = Some(description.into());
        self
    }

    #[must_use]
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.meta.url = Some(url.into());
        self
    }

    #[must_use]
    pub fn created_at(mut self, created_at: impl Into<String>) -> Self {
        self.meta.created_at = Some(created_at.into());
        self
    }

    #[must_use]
    pub fn updated_at(mut self, updated_at: impl Into<String>) -> Self {
        self.meta.updated_at = Some(updated_at.into());
        self
    }
}

impl IssueBuilder<Set<IssueId>, Set<String>, Set<String>> {
    pub fn build(self) -> Issue {
        Issue {
            id: self.id.0,
            title: self.title.0,
            status: self.status.0,
            meta: self.meta,
        }
    }
}

#[derive(Clone, Debug)]
pub struct IssueDraft {
    team: TeamId,
    title: String,
    status: Option<String>,
    category: Option<StatusCategory>,
    project: Option<ProjectId>,
    milestone: Option<MilestoneId>,
    assignee: Option<UserId>,
    priority: Option<u8>,
}

impl IssueDraft {
    pub fn team(&self) -> &TeamId {
        &self.team
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn status(&self) -> Option<&str> {
        self.status.as_deref()
    }

    pub fn category(&self) -> Option<StatusCategory> {
        self.category
    }

    pub fn project(&self) -> Option<&ProjectId> {
        self.project.as_ref()
    }

    pub fn milestone(&self) -> Option<&MilestoneId> {
        self.milestone.as_ref()
    }

    pub fn assignee(&self) -> Option<&UserId> {
        self.assignee.as_ref()
    }

    pub fn priority(&self) -> Option<u8> {
        self.priority
    }
}

pub struct IssueDraftBuilder<M, T> {
    team: M,
    title: T,
    status: Option<String>,
    category: Option<StatusCategory>,
    project: Option<ProjectId>,
    milestone: Option<MilestoneId>,
    assignee: Option<UserId>,
    priority: Option<u8>,
}

pub fn issue_draft() -> IssueDraftBuilder<Missing, Missing> {
    IssueDraftBuilder {
        team: Missing,
        title: Missing,
        status: None,
        category: None,
        project: None,
        milestone: None,
        assignee: None,
        priority: None,
    }
}

impl<T> IssueDraftBuilder<Missing, T> {
    pub fn team(self, team: impl Into<TeamId>) -> IssueDraftBuilder<Set<TeamId>, T> {
        IssueDraftBuilder {
            team: Set(team.into()),
            title: self.title,
            status: self.status,
            category: self.category,
            project: self.project,
            milestone: self.milestone,
            assignee: self.assignee,
            priority: self.priority,
        }
    }
}

impl<M> IssueDraftBuilder<M, Missing> {
    pub fn title(self, title: impl Into<String>) -> IssueDraftBuilder<M, Set<String>> {
        IssueDraftBuilder {
            team: self.team,
            title: Set(title.into()),
            status: self.status,
            category: self.category,
            project: self.project,
            milestone: self.milestone,
            assignee: self.assignee,
            priority: self.priority,
        }
    }
}

impl<M, T> IssueDraftBuilder<M, T> {
    #[must_use]
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    #[must_use]
    pub fn category(mut self, category: StatusCategory) -> Self {
        self.category = Some(category);
        self
    }

    #[must_use]
    pub fn project(mut self, project: impl Into<ProjectId>) -> Self {
        self.project = Some(project.into());
        self
    }

    #[must_use]
    pub fn milestone(mut self, milestone: impl Into<MilestoneId>) -> Self {
        self.milestone = Some(milestone.into());
        self
    }

    #[must_use]
    pub fn assignee(mut self, assignee: impl Into<UserId>) -> Self {
        self.assignee = Some(assignee.into());
        self
    }

    #[must_use]
    pub fn priority(mut self, priority: u8) -> Self {
        self.priority = Some(priority);
        self
    }
}

impl IssueDraftBuilder<Set<TeamId>, Set<String>> {
    pub fn build(self) -> IssueDraft {
        IssueDraft {
            team: self.team.0,
            title: self.title.0,
            status: self.status,
            category: self.category,
            project: self.project,
            milestone: self.milestone,
            assignee: self.assignee,
            priority: self.priority,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct IssuePatch {
    title: Option<String>,
    status: Option<String>,
    category: Option<StatusCategory>,
    project: Option<ProjectId>,
    milestone: Option<MilestoneId>,
    assignee: Option<UserId>,
    priority: Option<u8>,
}

impl IssuePatch {
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    pub fn status(&self) -> Option<&str> {
        self.status.as_deref()
    }

    pub fn category(&self) -> Option<StatusCategory> {
        self.category
    }

    pub fn project(&self) -> Option<&ProjectId> {
        self.project.as_ref()
    }

    pub fn milestone(&self) -> Option<&MilestoneId> {
        self.milestone.as_ref()
    }

    pub fn assignee(&self) -> Option<&UserId> {
        self.assignee.as_ref()
    }

    pub fn priority(&self) -> Option<u8> {
        self.priority
    }

    pub fn is_empty(&self) -> bool {
        self.title.is_none()
            && self.status.is_none()
            && self.category.is_none()
            && self.project.is_none()
            && self.milestone.is_none()
            && self.assignee.is_none()
            && self.priority.is_none()
    }
}

#[derive(Default)]
pub struct IssuePatchBuilder {
    patch: IssuePatch,
}

pub fn issue_patch() -> IssuePatchBuilder {
    IssuePatchBuilder::default()
}

impl IssuePatchBuilder {
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.patch.title = Some(title.into());
        self
    }

    #[must_use]
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.patch.status = Some(status.into());
        self
    }

    #[must_use]
    pub fn category(mut self, category: StatusCategory) -> Self {
        self.patch.category = Some(category);
        self
    }

    #[must_use]
    pub fn project(mut self, project: impl Into<ProjectId>) -> Self {
        self.patch.project = Some(project.into());
        self
    }

    #[must_use]
    pub fn milestone(mut self, milestone: impl Into<MilestoneId>) -> Self {
        self.patch.milestone = Some(milestone.into());
        self
    }

    #[must_use]
    pub fn assignee(mut self, assignee: impl Into<UserId>) -> Self {
        self.patch.assignee = Some(assignee.into());
        self
    }

    #[must_use]
    pub fn priority(mut self, priority: u8) -> Self {
        self.patch.priority = Some(priority);
        self
    }

    pub fn build(self) -> IssuePatch {
        self.patch
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Comment {
    id: CommentId,
    body: String,
    author: Option<UserId>,
    created_at: Option<String>,
}

impl Comment {
    pub fn id(&self) -> &CommentId {
        &self.id
    }

    pub fn body(&self) -> &str {
        &self.body
    }

    pub fn author(&self) -> Option<&UserId> {
        self.author.as_ref()
    }

    pub fn created_at(&self) -> Option<&str> {
        self.created_at.as_deref()
    }
}

pub struct CommentBuilder<I, B> {
    id: I,
    body: B,
    author: Option<UserId>,
    created_at: Option<String>,
}

pub fn comment() -> CommentBuilder<Missing, Missing> {
    CommentBuilder {
        id: Missing,
        body: Missing,
        author: None,
        created_at: None,
    }
}

impl<B> CommentBuilder<Missing, B> {
    pub fn id(self, id: impl Into<CommentId>) -> CommentBuilder<Set<CommentId>, B> {
        CommentBuilder {
            id: Set(id.into()),
            body: self.body,
            author: self.author,
            created_at: self.created_at,
        }
    }
}

impl<I> CommentBuilder<I, Missing> {
    pub fn body(self, body: impl Into<String>) -> CommentBuilder<I, Set<String>> {
        CommentBuilder {
            id: self.id,
            body: Set(body.into()),
            author: self.author,
            created_at: self.created_at,
        }
    }
}

impl<I, B> CommentBuilder<I, B> {
    #[must_use]
    pub fn author(mut self, author: impl Into<UserId>) -> Self {
        self.author = Some(author.into());
        self
    }

    #[must_use]
    pub fn created_at(mut self, created_at: impl Into<String>) -> Self {
        self.created_at = Some(created_at.into());
        self
    }
}

impl CommentBuilder<Set<CommentId>, Set<String>> {
    pub fn build(self) -> Comment {
        Comment {
            id: self.id.0,
            body: self.body.0,
            author: self.author,
            created_at: self.created_at,
        }
    }
}

macro_rules! named_entity {
    ($name:ident, $id:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
        pub struct $name {
            id: $id,
            name: String,
        }

        impl $name {
            pub fn make(id: $id, name: impl Into<String>) -> Self {
                Self {
                    id,
                    name: name.into(),
                }
            }

            pub fn id(&self) -> &$id {
                &self.id
            }

            pub fn name(&self) -> &str {
                &self.name
            }
        }
    };
}

named_entity!(Project, ProjectId);
named_entity!(Milestone, MilestoneId);
named_entity!(Cycle, CycleId);
named_entity!(Team, TeamId);
named_entity!(User, UserId);
named_entity!(Label, LabelId);
