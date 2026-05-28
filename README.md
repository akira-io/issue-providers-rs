<p align="center">
  <img src="assets/banner.svg" alt="omnitrack" />
</p>

<p align="center">
  <a href="https://crates.io/crates/omnitrack"><img src="https://img.shields.io/crates/v/omnitrack.svg" alt="crates.io"></a>
  <a href="https://crates.io/crates/omnitrack"><img src="https://img.shields.io/crates/d/omnitrack.svg" alt="downloads"></a>
  <a href="https://docs.rs/omnitrack"><img src="https://img.shields.io/docsrs/omnitrack" alt="docs.rs"></a>
  <a href="https://github.com/akira-io/omnitrack-rs/actions/workflows/test.yml"><img src="https://github.com/akira-io/omnitrack-rs/actions/workflows/test.yml/badge.svg" alt="tests"></a>
  <img src="https://img.shields.io/crates/l/omnitrack.svg" alt="license">
  <img src="https://img.shields.io/badge/MSRV-1.89-blue" alt="MSRV">
</p>

`omnitrack` is a universal issue-tracker provider abstraction for Rust. Linear and Jira ship as
driver modules behind feature flags inside a single crate; provider-neutral contracts (issues,
comments, labels, milestones, projects, teams, cycles, registry, pagination, errors) live at the
crate root so consumers code against one surface regardless of the backend.

## Install

CLI:

```sh
# Default: Linear driver
cargo add omnitrack

# Jira only
cargo add omnitrack --no-default-features --features jira

# All providers
cargo add omnitrack --features all

# Contracts only, no driver
cargo add omnitrack --no-default-features
```

By hand in `Cargo.toml`:

```toml
omnitrack = "0.3"
omnitrack = { version = "0.3", default-features = false, features = ["jira"] }
omnitrack = { version = "0.3", features = ["all"] }
omnitrack = { version = "0.3", default-features = false }
```

## Quick start

```rust,no_run
use omnitrack::{IssueFilter, IssueResult, Issues, provider};
use omnitrack::linear::linear;

#[tokio::main]
async fn main() -> IssueResult<()> {
    let registry = provider().register(linear())?.build();
    for descriptor in registry.descriptors() {
        println!("{} ({})", descriptor.display_name(), descriptor.id().as_str());
    }

    let token = std::env::var("LINEAR_TOKEN").unwrap_or_default();
    let client = linear().token(token).build();
    let page = client.list(IssueFilter::default(), None).await?;
    println!("fetched {} issue(s)", page.items().len());
    Ok(())
}
```

## Documentation

Full documentation lives in this repository under `docs/`. Each driver also ships its own usage
guide:

- Core contracts: `docs/00-overview.md`
- Linear driver: `docs/linear/`
- Jira driver: `docs/jira/`
- API reference on docs.rs: https://docs.rs/omnitrack

## Testing

```sh
cargo test --all-features
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.
