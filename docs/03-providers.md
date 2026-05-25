# Providers

## Anatomy of a provider crate

A provider crate exposes a zero-config struct, an `impl Provider`, and a free constructor helper:

```rust
pub const PROVIDER_ID: &str = "linear";
pub const DISPLAY_NAME: &str = "Linear";

#[derive(Clone, Copy, Debug, Default)]
pub struct LinearProvider;

impl Provider for LinearProvider {
    fn descriptor(&self) -> ProviderDescriptor {
        ProviderDescriptorBuilder::make(PROVIDER_ID, DISPLAY_NAME)
            .capability(Capability::Issues)
            .capability(Capability::Projects)
            // …
            .build()
    }
}

pub fn linear() -> LinearProvider { LinearProvider }
```

The helper (`linear()`) is what consumers pass to `provider().register(...)`.

## Adding a provider (e.g. Jira)

1. `crates/jira` with `issue-provider-core` as the only core dependency.
2. `JiraProvider` + `impl Provider` declaring its capabilities.
3. `pub fn jira() -> JiraProvider`.
4. Add the package to `test.yml`'s `providers` matrix and `publish.yml`.

No core edits. No central match statement.

## Transport

`linear()` is the descriptor (for the registry). To call capabilities, build a credentialed client:

```rust
use issue_provider_core::{IssueId, Issues};
use issue_provider_linear::linear;

let client = linear().token("lin_api_...").build();   // LinearClient
let page = client.list(None).await?;                   // Issues::list
let one = client.get(IssueId::make("ISS-1")).await?;   // Issues::get
```

The GraphQL transport (auth, pagination, retry on transient/empty responses) stays private to the provider crate; only normalized models cross the boundary. Linear's `state.type` maps to `StatusCategory` via `category_from_state_type`.

`LinearClient` implements every declared capability: `Issues` (rich mapping), the named-entity capabilities `Projects`, `Milestones`, `Cycles`, `Teams`, `Users`, `Labels` (each `get`/`list` over Linear's `id name` nodes with cursor pagination), `Viewer` (`current_user` via Linear's `viewer { id name }`, for credential validation), and `Comments` (`list_comments`/`post_comment`/`delete_comment` via Linear's issue `comments` connection and `commentCreate`/`commentDelete`).

`Issues` also covers mutations:

```rust
use issue_provider_core::{Issues, StatusCategory, issue_draft, issue_patch};

let created = client.create(issue_draft().team("TEAM_ID").title("Bug").build()).await?;
let updated = client.update(created.id().clone(), issue_patch().priority(1).build()).await?;
client.close(updated.id().clone()).await?;  // sugar: update(category = Completed)
client.delete(created.id().clone()).await?;
```

Linear has no generic "close" and addresses workflow states by id, so `create`/`update` resolve a `StatusCategory` to a concrete `stateId`: the client looks up the issue's team workflow states and picks the first whose `type` matches (`state_type_for`). `create` requires `team` because `issueCreate` requires `teamId`.

---

Prev: [Contracts](./02-contracts.md) · [Index](./README.md) · Next: [Development](./04-development.md)
