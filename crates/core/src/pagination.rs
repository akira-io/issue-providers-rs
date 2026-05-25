use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PageCursor(String);

impl PageCursor {
    pub fn make(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct PageRequest {
    after: Option<PageCursor>,
    limit: Option<u32>,
}

impl PageRequest {
    pub fn after(&self) -> Option<&PageCursor> {
        self.after.as_ref()
    }

    pub fn limit(&self) -> Option<u32> {
        self.limit
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Page<T> {
    items: Vec<T>,
    next: Option<PageCursor>,
}

impl<T> Page<T> {
    pub fn make(items: Vec<T>, next: Option<PageCursor>) -> Self {
        Self { items, next }
    }

    pub fn items(&self) -> &[T] {
        &self.items
    }

    pub fn into_items(self) -> Vec<T> {
        self.items
    }

    pub fn next(&self) -> Option<&PageCursor> {
        self.next.as_ref()
    }

    pub fn has_next(&self) -> bool {
        self.next.is_some()
    }
}

#[derive(Clone, Debug, Default)]
pub struct PaginationBuilder {
    after: Option<PageCursor>,
    limit: Option<u32>,
}

impl PaginationBuilder {
    #[must_use]
    pub fn after(mut self, cursor: PageCursor) -> Self {
        self.after = Some(cursor);
        self
    }

    #[must_use]
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn build(self) -> PageRequest {
        PageRequest {
            after: self.after,
            limit: self.limit,
        }
    }
}

pub fn pagination() -> PaginationBuilder {
    PaginationBuilder::default()
}
