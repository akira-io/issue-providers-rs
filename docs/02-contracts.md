# Contracts

## Capability traits

Every capability follows the same shape:

```rust
pub trait Issues: Send + Sync {
    fn get(&self, id: IssueId) -> BoxFuture<'_, IssueResult<Issue>>;
    fn list(&self, page: Option<PageRequest>) -> BoxFuture<'_, IssueResult<Page<Issue>>>;
}
```

Same for `Projects`, `Milestones`, `Cycles`, `Teams`, `Users`, `Labels` over their respective entity + id. Each has a `TransportNotConfigured*` default impl returning `ErrorKind::TransportNotConfigured`.

## Value objects

```rust
let it = issue()
    .id("ISS-1")                  // required
    .title("Title")               // required
    .status("open")               // required (raw provider status)
    .category(StatusCategory::Started)  // normalized lifecycle
    .project("PRJ-1")
    .milestone("MIL-1")
    .assignee("USR-1")
    .priority(2)
    .updated_at("2026-05-25T00:00:00Z")
    .build();
```

`issue()` is a typestate builder: `.build()` only exists once `id`, `title`, and `status` are set (compile-time enforced, like `vcs-providers-rs`). Optional fields default to `None` / empty.

Id setters (`id`, `project`, `milestone`, `assignee`) accept either a raw string or the newtype — both compile, since the id newtypes implement `From<&str>` / `From<String>`:

```rust
issue().id("ISS-1")                  // raw string
issue().id(IssueId::make("ISS-1"))   // newtype — both work
```

The built `Issue` always stores strong newtypes; the string form is convenience sugar.

`status` is the raw provider status string (varies per tracker). `category` is the normalized lifecycle for cross-provider filtering:

```rust
pub enum StatusCategory { Backlog, Unstarted, Started, Completed, Canceled }
```

Linear `state.type`, Jira `statusCategory`, and GitHub open/closed all map onto it. `category` is optional until a provider maps it.

Named entities (`Project`, `Milestone`, `Cycle`, `Team`, `User`, `Label`) are `make(id, name)` with `id()` / `name()` accessors. All ids are newtypes.

## Pagination

```rust
let req = pagination().after(PageCursor::make("c1")).limit(100).build();
let page: Page<Issue> = /* provider.list(Some(req)) */;
for issue in page.items() { /* … */ }
let more = page.next(); // Option<&PageCursor>
```

## Errors

```rust
pub enum ErrorKind { TransportNotConfigured, Transport, Decode, NotFound, Unauthorized, RateLimited, Provider }
let err = error().of(ErrorKind::NotFound, "issue missing");
```

---

Prev: [Architecture](./01-architecture.md) · [Index](./README.md) · Next: [Providers](./03-providers.md)
