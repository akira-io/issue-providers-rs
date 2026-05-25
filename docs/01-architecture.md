# Architecture

## Workspace layout

```text
issue-providers-rs/
├── crates/
│   ├── core/      # issue-provider-core — neutral contracts + builders
│   └── linear/    # issue-provider-linear — Linear driver
└── examples/      # fluent-builder usage
```

Each provider is its own crate depending only on `issue-provider-core`. Adding Jira means adding `crates/jira` with a `jira()` helper — no edits to core.

## Composition: fluent registry

Providers register into a registry through a fluent builder. The entry point is `provider()` in core; each provider crate exposes a free constructor (`linear()`, later `jira()`):

```rust
use issue_provider_core::{provider, IssueResult};
use issue_provider_linear::linear;

let registry = provider()
    .register(linear())?
    // .register(jira())?
    .build();
```

`register` rejects duplicate provider ids. `descriptors()` reports each provider's id, display name, and capabilities.

## Capabilities

A provider declares which capabilities it supports via its `ProviderDescriptor`. Each capability is a provider-neutral trait:

`Issues`, `Projects`, `Milestones`, `Cycles`, `Teams`, `Users`, `Labels`.

Every capability trait offers `get(id)` and `list(page)` returning `BoxFuture<IssueResult<…>>`. A `TransportNotConfigured*` default implementation returns a `TransportNotConfigured` error, so a provider compiles before its transport is wired.

## Normalized models

Resources are immutable value objects with `make()` constructors and accessors. `Issue` adds fluent `with_project / with_milestone / with_assignee / with_priority` setters. Ids are newtypes (`IssueId`, `ProjectId`, …) to avoid mixing identifiers across entities.

## Pagination

`list` takes `Option<PageRequest>` (built via `pagination().after(cursor).limit(n).build()`) and returns `Page<T>` with `items()` and an optional `next()` cursor. Providers translate this to their native cursor/offset scheme.

## Errors

`IssueError { kind, message }` with `ErrorKind` (`TransportNotConfigured`, `Transport`, `Decode`, `NotFound`, `Unauthorized`, `RateLimited`, `Provider`). Built via `error().of(kind, msg)`. `IssueResult<T>` is the crate result alias.

## Persistence boundary

The crate fetches and normalizes only. The consumer (e.g. unified-dev) maps normalized models into its own storage and runs its own sync worker, cursors, and commands. This mirrors how `akira-billing` is an API client while the app owns storage.

## Roadmap (post-P1)

- P2: real GraphQL transport for Linear + capability implementations + delta queries.
- P3: Jira driver.
- Optional: middleware, rate-limit observation, telemetry (mirroring `vcs-providers-rs`).

---

Prev: [Overview](./00-overview.md) · [Index](./README.md) · Next: [Contracts](./02-contracts.md)
