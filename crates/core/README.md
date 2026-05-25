# issue-provider-core

Provider-neutral contracts for issue trackers (Linear, Jira, ...). The shared foundation
for the [`issue-providers-rs`](https://github.com/akira-io/issue-providers-rs) workspace.

Defines the normalized models, capability traits, error/pagination types, and the provider
registry. Driver crates (e.g. `issue-provider-linear`) depend on this and implement the
capabilities.

```rust
use issue_provider_core::{issue, StatusCategory};

let item = issue()
    .id("ISS-1")
    .title("Fix login")
    .status("In Progress")
    .category(StatusCategory::Started)
    .build();
```

## Capabilities

`Issues`, `Projects`, `Milestones`, `Cycles`, `Teams`, `Users`, `Labels` — each a trait with
`get` / `list` returning paginated, normalized results.

## License

MIT OR Apache-2.0.
