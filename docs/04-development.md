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
- `bump.yml` — manual (`workflow_dispatch`): pick a crate (or `all`) and a bump level; computes the next version and pushes the release tag.
- `release.yml` — on tag `<crate>-vX.Y.Z` or `workspace-vX.Y.Z`: derives the package(s) + version from the tag, runs `cargo set-version` (bumps the crate(s) and any intra-workspace dependents' requirement), regenerates a **per-crate** `crates/<crate>/CHANGELOG.md` via git-cliff (`--include-path crates/<crate>/**`), commits back to the default branch, creates the GitHub Release, and posts it to Discord. Commit author is the tag pusher (`GITHUB_ACTOR`), not a bot.
- `publish.yml` — on tag `<crate>-vX.Y.Z` or `workspace-vX.Y.Z`: crates.io Trusted Publishing, runs `cargo set-version` from the tag then publishes the resolved crate(s) in dependency order (retrying so a provider waits for a freshly-published `issue-provider-core` to be index-resolvable). Trigger is `push` on tags (never `workflow_run`, which Trusted Publishing rejects).

## Release flow

Crates are versioned independently (not lockstep), but a single workspace-wide release is also supported. The tag is the source of truth — both workflows derive the package(s) and version from it, so no pre-bump PR is needed.

Tag formats:

- `<short>-vX.Y.Z` — release one crate (`<short>` maps to `issue-provider-<short>`: `core`, `linear`, `jira`).
- `workspace-vX.Y.Z` — bump every publishable crate to `X.Y.Z` and publish them in dependency order (`core` first).

```bash
git tag core-v0.1.1        # release issue-provider-core 0.1.1
git tag linear-v0.2.0      # release issue-provider-linear 0.2.0
git tag workspace-v0.3.0   # release every crate at 0.3.0
```

Push the tag; CI sets the version, writes the changelog, publishes the GitHub Release, and pushes to crates.io. No manual version edits.

### Automated bump

The `bump` workflow (Actions → bump → Run workflow) does it without local git: pick a `target` (`core` / `linear` / `jira` / `all`) and a `level` (`patch` / `minor` / `major`). It computes the next version and pushes the matching tag, which triggers `release` + `publish`.

Ordering: when a provider release needs a new `issue-provider-core`, release `core-v…` first; the provider's publish step retries until the new core is resolvable on the index. A `workspace-v…` release handles ordering automatically.

### Changelog and notifications

- **Changelog** is per-crate: each release writes `crates/<crate>/CHANGELOG.md`, scoped to commits touching that crate's path via git-cliff `--include-path`. A `workspace-v…` release writes every crate's changelog.
- **Discord** announcements are posted on each GitHub Release via `SethCohen/github-releases-to-discord`. Add a channel webhook URL as the repo secret `DISCORD`; the step is skipped when the secret is unset.

---

Prev: [Providers](./03-providers.md) · [Index](./README.md)
