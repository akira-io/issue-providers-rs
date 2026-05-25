# Changelog

All notable changes to this project are documented in this file.

## [workspace-v0.2.0] - 2026-05-25


### Code Refactoring

- Issue() builder accepts raw string ids ([28cb5c9](akira-io/issue-providers-rs/commit/28cb5c953d50be223f636fca5bc14f5688cd7ba0))



### Continuous Integration

- Independent per-crate tag-driven versioning ([8715d24](akira-io/issue-providers-rs/commit/8715d24248980d7c3d6e049162697bf2326d5c10))



### Features

- Scaffold issue-providers-rs with core and linear driver ([8a01a11](akira-io/issue-providers-rs/commit/8a01a114415dbbf7c10d8d60fbedd87f552c5d51))

- Add normalized StatusCategory to Issue ([498b963](akira-io/issue-providers-rs/commit/498b9638492221e09e16f2acd8201e69c8a40f1e))

- Builder id setters accept string or newtype ([5171a3c](akira-io/issue-providers-rs/commit/5171a3c31c113d34d245f281b9fe6869afbb6121))

- Add issue mutation verbs (create/update/delete/close) ([f2c9ec3](akira-io/issue-providers-rs/commit/f2c9ec3eb0b034eaeddc9733a236b3cfee803e66))

- Fluent executing terminals on draft/patch builders ([20c895e](akira-io/issue-providers-rs/commit/20c895e16cb80b9d3529337a97e5aa1eceaff3f0))

- Enrich Issue with identifier, description, url, author, team, labels, timestamps ([7f0f60b](akira-io/issue-providers-rs/commit/7f0f60b15daab1b467409edffa6a8839c3949ab7))

- Add Viewer capability for authenticated account ([2317cbf](akira-io/issue-providers-rs/commit/2317cbf332ff5c065a97dfb1a2615be5582167e9))

- Add Comments capability ([9f54ffa](akira-io/issue-providers-rs/commit/9f54ffa3e79f73f3689e8e3b2e04454fed509867))

- Filter on Issues::list ([a70ad38](akira-io/issue-providers-rs/commit/a70ad38c934ac2593ad040650423861a3aaad8cf))



### Maintenance

- Publish-ready metadata, readmes, and credentialed example ([324330e](akira-io/issue-providers-rs/commit/324330ea0acdafba4af4698e459ebdc20c809c4d))

- Workspace-v0.2.0 ([cc9d635](akira-io/issue-providers-rs/commit/cc9d635d57007109bc368ed9f4f477e2584ba29c))



### Testing

- Add serde round-trip coverage for models and StatusCategory ([ba16e8e](akira-io/issue-providers-rs/commit/ba16e8e6a72740983885173c56b8ff7e5ad1f28d))



