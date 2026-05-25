# issue-providers-rs

Universal async-first issue-tracker provider abstraction layer for Rust.

This is the issue-tracker counterpart to `vcs-providers-rs`. It is not a Linear or Jira SDK; Linear, Jira, and future trackers are driver implementations behind provider-neutral contracts.

## Layout

```text
issue-providers-rs/
├── crates/
│   ├── core/      # issue-provider-core — neutral contracts (Issue, Project, Milestone, ...)
│   └── linear/    # issue-provider-linear — Linear driver
└── examples/
```

## Usage

```rust
use issue_provider_core::{provider, IssueResult};
use issue_provider_linear::linear;

fn main() -> IssueResult<()> {
    let registry = provider()
        .register(linear())?
        // .register(jira())?   // later
        .build();

    for descriptor in registry.descriptors() {
        println!("{} ({})", descriptor.display_name(), descriptor.id().as_str());
    }

    Ok(())
}
```

To call a provider, build a credentialed client:

```rust
use issue_provider_core::{IssueId, Issues};
use issue_provider_linear::linear;

let client = linear().token("lin_api_...").build();
let page = client.list(None).await?;                 // Issues::list
let one = client.get(IssueId::make("ISS-1")).await?; // Issues::get
```

`Issues` also covers mutations (`create`, `update`, `delete`, `close`):

```rust
use issue_provider_core::{Issues, issue_draft, issue_patch};

let created = client.create(issue_draft().team("TEAM_ID").title("Bug").build()).await?;
client.update(created.id().clone(), issue_patch().priority(1).build()).await?;
client.close(created.id().clone()).await?; // sugar over update(category = Completed)
client.delete(created.id().clone()).await?;
```

The draft/patch builders also offer an executing terminal:

```rust
let created = issue_draft().team("TEAM_ID").title("Bug").create(&client).await?;
let updated = issue_patch().priority(1).update(&client, created.id().clone()).await?;
```

## Capabilities

`Issues` (`get` / `list` / `create` / `update` / `delete` / `close`) plus read-only `Projects`, `Milestones`, `Cycles`, `Teams`, `Users`, `Labels` (`get` / `list`) — each a provider-neutral trait returning paginated, normalized results. Persistence (SQLite, etc.) is the consumer's responsibility; this crate only fetches and normalizes.

See [docs/](docs/README.md) for architecture, contracts, and provider authoring.

## License

MIT OR Apache-2.0
