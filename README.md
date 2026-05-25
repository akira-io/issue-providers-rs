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

## Capabilities

`Issues`, `Projects`, `Milestones`, `Cycles`, `Teams`, `Users`, `Labels` — each a provider-neutral trait with `get` / `list` returning paginated results. Persistence (SQLite, etc.) is the consumer's responsibility; this crate only fetches and normalizes.

## License

MIT OR Apache-2.0
