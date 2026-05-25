# Development

## Toolchain

Pinned via `rust-toolchain.toml`: Rust `1.89.0`, edition 2024, components `clippy` + `rustfmt`.

## Lints (workspace, strict)

- `unsafe_code = "forbid"`
- clippy `unwrap_used`, `expect_used`, `panic`, `todo` = `deny`

Tests must avoid `.unwrap()` / `.expect()` / `panic!` — return `IssueResult<()>` and use `?`, or `assert!` / `assert_eq!`.

## Commands

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

## CI (`.github/workflows`)

- `test.yml` — matrix ubuntu/macos/windows: fmt + test + clippy `-D`; plus a per-provider package job.
- `release.yml` — on tag `<crate>-vX.Y.Z`: derives the package + version from the tag, runs `cargo set-version -p <pkg>` (bumps that crate and any intra-workspace dependents' requirement on it), regenerates `CHANGELOG.md` via git-cliff, commits back to the default branch, creates the GitHub Release. Commit author is the tag pusher (`GITHUB_ACTOR`), not a bot.
- `publish.yml` — on tag `<crate>-vX.Y.Z`: crates.io Trusted Publishing, runs `cargo set-version` from the tag then publishes that one crate (retrying so a provider waits for a freshly-published `issue-provider-core` to be index-resolvable). Trigger is `push` on tags (never `workflow_run`, which Trusted Publishing rejects).

## Release flow

Each crate is versioned and released independently (not lockstep). The tag is the source of truth — both workflows derive the package and version from it, so no pre-bump PR is needed. Tag format: `<short>-vX.Y.Z`, where `<short>` maps to `issue-provider-<short>` (`core`, `linear`, `jira`).

```bash
git tag core-v0.1.1      # release issue-provider-core 0.1.1
git push origin core-v0.1.1

git tag linear-v0.2.0    # release issue-provider-linear 0.2.0
git push origin linear-v0.2.0
```

CI sets the version, writes the changelog, publishes the GitHub Release, and pushes the crate to crates.io. No manual version edits.

Ordering: when a provider release needs a new `issue-provider-core`, tag and release `core-v…` first; the provider's `publish.yml` retries until the new core is resolvable on the index.

---

Prev: [Providers](./03-providers.md) · [Index](./README.md)
