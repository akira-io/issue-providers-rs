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
- `release.yml` — on tag `vX.Y.Z`: sets `[workspace.package].version` from the tag, regenerates `CHANGELOG.md` via git-cliff, commits version + changelog back to the default branch, creates the GitHub Release. Commit author is the tag pusher (`GITHUB_ACTOR`), not a bot.
- `publish.yml` — on tag `vX.Y.Z`: crates.io Trusted Publishing, publishes `issue-provider-core` then `issue-provider-linear`. Trigger is `push` on tags (never `workflow_run`, which Trusted Publishing rejects).

## Release flow

The pipeline is fully tag-driven. To cut a release:

```bash
git tag vX.Y.Z
git push origin vX.Y.Z
```

CI bumps the toml, writes the changelog, publishes the GitHub Release, and pushes the crates to crates.io. No manual version edits.

---

Prev: [Providers](./03-providers.md) · [Index](./README.md)
