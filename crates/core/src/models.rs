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
    };
}

id_newtype!(IssueId);
id_newtype!(ProjectId);
id_newtype!(MilestoneId);
id_newtype!(CycleId);
id_newtype!(TeamId);
id_newtype!(UserId);
id_newtype!(LabelId);

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum StatusCategory {
    Backlog,
    Unstarted,
    Started,
    Completed,
    Canceled,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Issue {
    id: IssueId,
    title: String,
    status: String,
    category: Option<StatusCategory>,
    project: Option<ProjectId>,
    milestone: Option<MilestoneId>,
    assignee: Option<UserId>,
    priority: Option<u8>,
    updated_at: String,
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

    pub fn updated_at(&self) -> &str {
        &self.updated_at
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
    category: Option<StatusCategory>,
    project: Option<String>,
    milestone: Option<String>,
    assignee: Option<String>,
    priority: Option<u8>,
    updated_at: Option<String>,
}

pub fn issue() -> IssueBuilder<Missing, Missing, Missing> {
    IssueBuilder {
        id: Missing,
        title: Missing,
        status: Missing,
        category: None,
        project: None,
        milestone: None,
        assignee: None,
        priority: None,
        updated_at: None,
    }
}

impl<T, S> IssueBuilder<Missing, T, S> {
    pub fn id(self, id: impl Into<String>) -> IssueBuilder<Set<String>, T, S> {
        IssueBuilder {
            id: Set(id.into()),
            title: self.title,
            status: self.status,
            category: self.category,
            project: self.project,
            milestone: self.milestone,
            assignee: self.assignee,
            priority: self.priority,
            updated_at: self.updated_at,
        }
    }
}

impl<I, S> IssueBuilder<I, Missing, S> {
    pub fn title(self, title: impl Into<String>) -> IssueBuilder<I, Set<String>, S> {
        IssueBuilder {
            id: self.id,
            title: Set(title.into()),
            status: self.status,
            category: self.category,
            project: self.project,
            milestone: self.milestone,
            assignee: self.assignee,
            priority: self.priority,
            updated_at: self.updated_at,
        }
    }
}

impl<I, T> IssueBuilder<I, T, Missing> {
    pub fn status(self, status: impl Into<String>) -> IssueBuilder<I, T, Set<String>> {
        IssueBuilder {
            id: self.id,
            title: self.title,
            status: Set(status.into()),
            category: self.category,
            project: self.project,
            milestone: self.milestone,
            assignee: self.assignee,
            priority: self.priority,
            updated_at: self.updated_at,
        }
    }
}

impl<I, T, S> IssueBuilder<I, T, S> {
    #[must_use]
    pub fn category(mut self, category: StatusCategory) -> Self {
        self.category = Some(category);
        self
    }

    #[must_use]
    pub fn project(mut self, project: impl Into<String>) -> Self {
        self.project = Some(project.into());
        self
    }

    #[must_use]
    pub fn milestone(mut self, milestone: impl Into<String>) -> Self {
        self.milestone = Some(milestone.into());
        self
    }

    #[must_use]
    pub fn assignee(mut self, assignee: impl Into<String>) -> Self {
        self.assignee = Some(assignee.into());
        self
    }

    #[must_use]
    pub fn priority(mut self, priority: u8) -> Self {
        self.priority = Some(priority);
        self
    }

    #[must_use]
    pub fn updated_at(mut self, updated_at: impl Into<String>) -> Self {
        self.updated_at = Some(updated_at.into());
        self
    }
}

impl IssueBuilder<Set<String>, Set<String>, Set<String>> {
    pub fn build(self) -> Issue {
        Issue {
            id: IssueId::make(self.id.0),
            title: self.title.0,
            status: self.status.0,
            category: self.category,
            project: self.project.map(ProjectId::make),
            milestone: self.milestone.map(MilestoneId::make),
            assignee: self.assignee.map(UserId::make),
            priority: self.priority,
            updated_at: self.updated_at.unwrap_or_default(),
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
