# issue-provider-linear

Linear driver for [`issue-provider-core`](https://crates.io/crates/issue-provider-core), part
of the [`issue-providers-rs`](https://github.com/akira-io/issue-providers-rs) workspace.

Talks to Linear's GraphQL API and returns the neutral `issue-provider-core` models. The
transport (auth, pagination, retry) stays private; only normalized models cross the boundary.

```rust
use issue_provider_core::{IssueId, Issues};
use issue_provider_linear::linear;

let client = linear().token("lin_api_...").build();
let page = client.list(None).await?;                 // Issues::list
let one = client.get(IssueId::make("ISS-1")).await?; // Issues::get
```

Implements every capability: `Issues` (`get`/`list`/`create`/`update`/`delete`/`close`) plus
read-only `Projects`, `Milestones`, `Cycles`, `Teams`, `Users`, `Labels`. Linear's `state.type`
maps to `StatusCategory` via `category_from_state_type`; `create`/`update` resolve a
`StatusCategory` back to a Linear workflow `stateId` for the issue's team.

## License

MIT OR Apache-2.0.
