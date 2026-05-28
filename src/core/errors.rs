use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    TransportNotConfigured,
    Transport,
    Decode,
    NotFound,
    Unauthorized,
    RateLimited,
    Provider,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueError {
    kind: ErrorKind,
    message: String,
}

impl IssueError {
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for IssueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.message)
    }
}

impl std::error::Error for IssueError {}

pub type IssueResult<T> = Result<T, IssueError>;

#[derive(Clone, Debug, Default)]
pub struct ErrorBuilder;

impl ErrorBuilder {
    pub fn of(self, kind: ErrorKind, message: impl Into<String>) -> IssueError {
        IssueError {
            kind,
            message: message.into(),
        }
    }

    pub fn transport_not_configured(self) -> IssueError {
        self.of(
            ErrorKind::TransportNotConfigured,
            "transport not configured for this capability",
        )
    }
}

pub fn error() -> ErrorBuilder {
    ErrorBuilder
}
