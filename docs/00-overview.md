# Overview

`issue-providers-rs` is a universal abstraction layer for issue-tracker providers in Rust. It is the issue-tracker counterpart to `vcs-providers-rs`.

It is not a Linear or Jira SDK. Linear, Jira, and future trackers are provider implementations behind provider-neutral Rust contracts.

The public model uses universal terminology, so consumers write code once and switch trackers without rewrites:

| Universal concept | Linear | Jira | GitHub Issues |
| --- | --- | --- | --- |
| Issue | Issue | Issue | Issue |
| Project | Project | Project | Project |
| Milestone | Project milestone | Fix version | Milestone |
| Cycle | Cycle | Sprint | — |
| Team | Team | Board | — |
| Label | Label | Label | Label |
| User | User | User | User |

Provider-specific behavior stays inside provider crates. Core contracts stay provider-neutral and stable.

## Goals

- Provider isolation (each tracker is a driver crate).
- Transport isolation (HTTP/GraphQL lives behind contracts).
- Async-first APIs.
- Contract-first provider behavior.
- Runtime capability negotiation (a provider declares what it supports).
- Immutable, normalized resource modeling.
- Fluent, explicit builders — no hidden behavior.

## Non-goals

- Exposing provider HTTP clients as public API.
- Exposing raw provider payloads as public API.
- Reusing provider terminology in universal contracts.
- Owning persistence — the consumer maps normalized models into its own store (SQLite, etc.).
- Central switch statements that require editing core when adding a provider.

## Relationship to `vcs-providers-rs`

`vcs-providers-rs` covers version control (repos, branches, pull/merge requests, pipelines, releases). `issue-providers-rs` covers issue tracking (issues, projects, milestones, cycles). A consumer that needs both (e.g. unified-dev) depends on both crates.

---

[Index](./README.md) · Next: [Architecture](./01-architecture.md)
