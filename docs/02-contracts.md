# Contracts

## Capability traits

The named-entity capabilities follow the same read shape:

```rust
pub trait Projects: Send + Sync {
    fn get(&self, id: ProjectId) -> BoxFuture<'_, IssueResult<Project>>;
    fn list(&self, page: Option<PageRequest>) -> BoxFuture<'_, IssueResult<Page<Project>>>;
}
```

Same for `Milestones`, `Cycles`, `Teams`, `Users`, `Labels` over their respective entity + id.

`Viewer` is a single-method capability for the authenticated account, used to validate credentials:

```rust
pub trait Viewer: Send + Sync {
    fn current_user(&self) -> BoxFuture<'_, IssueResult<User>>;
}
```

`Comments` operates on an issue's discussion: `list_comments(issue, page)`, `post_comment(issue, body)`, `delete_comment(id)`. `Comment` is `{ id, body, author, created_at }`, built via `comment().id(..).body(..).build()`.

`Issues` adds mutation verbs on top of the read pair:

```rust
pub trait Issues: Send + Sync {
    fn get(&self, id: IssueId) -> BoxFuture<'_, IssueResult<Issue>>;
    fn list(&self, page: Option<PageRequest>) -> BoxFuture<'_, IssueResult<Page<Issue>>>;
    fn create(&self, draft: IssueDraft) -> BoxFuture<'_, IssueResult<Issue>>;
    fn update(&self, id: IssueId, patch: IssuePatch) -> BoxFuture<'_, IssueResult<Issue>>;
    fn delete(&self, id: IssueId) -> BoxFuture<'_, IssueResult<()>>;
    fn close(&self, id: IssueId) -> BoxFuture<'_, IssueResult<Issue>> { /* default: update(category = Completed) */ }
}
```

`close` has a default impl (sugar over `update` setting `StatusCategory::Completed`), so a provider only implements `get`/`list`/`create`/`update`/`delete`. Each capability has a `TransportNotConfigured*` default impl returning `ErrorKind::TransportNotConfigured`.

`create` takes an `IssueDraft` (server assigns the id, so `Issue` is not reused). Required: `team` + `title` (typestate-enforced). `update` takes an `IssuePatch` (all fields optional; an empty patch is a no-op fetch):

```rust
let draft = issue_draft().team("TEAM-1").title("New issue")
    .category(StatusCategory::Unstarted).build();
let patch = issue_patch().title("Renamed").category(StatusCategory::Started).build();
```

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

Beyond the fields above, `Issue` also carries `identifier` (human key, e.g. `ENG-123`), `description`, `url`, `created_at`, `author` (`UserId`), `team` (`TeamId`), and `labels` (`Vec<LabelId>`, set via `.labels([...])` or repeated `.label(...)`). All optional/empty by default; providers populate what they expose.

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
