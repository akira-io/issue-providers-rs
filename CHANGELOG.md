# Changelog

All notable changes to this project are documented in this file.

## [0.3.0] - 2026-05-28


### Code Refactoring

- Collapse workspace into single crate omnitrack ([#24](https://github.com/akira-io/git-cognition-rs/issues/24)) ([18cdb28](akira-io/git-cognition-rs/commit/18cdb281d84d41509fce0a2dce19bc68cb0bf006))



### Documentation

- Add credits and license sections, ship CONTRIBUTING/SECURITY/LICENSE files ([#25](https://github.com/akira-io/git-cognition-rs/issues/25)) ([c0e72d1](akira-io/git-cognition-rs/commit/c0e72d19d3413d2248067c08111d7f9145cbf216))



### Maintenance

- Jira-v0.5.0 ([01d13e0](akira-io/git-cognition-rs/commit/01d13e0e85442a902135098381b4670342911a8f))


## [jira-v0.5.0] - 2026-05-27


### Bug Fixes

- Bounded JQL + ADF markdown descriptions ([#23](https://github.com/akira-io/git-cognition-rs/issues/23)) ([7238414](akira-io/git-cognition-rs/commit/7238414eebd668f01206f4836bf8925892c059ec))



### Maintenance

- Jira-v0.3.0 ([84767fc](akira-io/git-cognition-rs/commit/84767fc82cbe2bff0edc87329c066ae8c341ef81))


## [jira-v0.3.0] - 2026-05-27


### Features

- Add accessible_resources for OAuth cloud-id discovery ([#22](https://github.com/akira-io/git-cognition-rs/issues/22)) ([36711ef](akira-io/git-cognition-rs/commit/36711efa2ec7f8a90da9aac9411254d9983c4898))



### Maintenance

- Jira-v0.2.0 ([e34d18c](akira-io/git-cognition-rs/commit/e34d18c006cdc90921a8160fa50db08812efd125))


## [jira-v0.2.0] - 2026-05-26


### Features

- Add OAuth bearer auth and api gateway base ([#21](https://github.com/akira-io/git-cognition-rs/issues/21)) ([627fe4c](akira-io/git-cognition-rs/commit/627fe4cdebf521c67cf2f63736fb95ec03872a9d))



### Maintenance

- Jira-v0.1.0 ([ef1848f](akira-io/git-cognition-rs/commit/ef1848fb1c00e0bc0df00af6a3b7be2296436f4e))


## [jira-v0.1.0] - 2026-05-26


### Continuous Integration

- Use semver version as changelog tag header ([f60dcd9](akira-io/git-cognition-rs/commit/f60dcd939f16f3fec79de6912dc8596c58d5c910))

- Pin actions to commit SHA and cover the jira crate ([6e35380](akira-io/git-cognition-rs/commit/6e3538046335fd41710b953e4e10fd918d89b2ff))

- Pin release, publish and bump actions to commit SHA ([#19](https://github.com/akira-io/git-cognition-rs/issues/19)) ([75d6425](akira-io/git-cognition-rs/commit/75d64254abd229024741e8b1cb62f4ef34baf117))



### Features

- Add Jira Cloud issue-provider driver ([f0b0dc2](akira-io/git-cognition-rs/commit/f0b0dc2e6e125b52a4c9d9b17074440ba36c1bb8))



### Maintenance

- Workspace-v0.2.0 ([606a5c4](akira-io/git-cognition-rs/commit/606a5c4dbd7204f40dbe61cbc447d94072aa66e5))

- Drop root changelog in favor of per-crate ([7791a13](akira-io/git-cognition-rs/commit/7791a13811e366947aad24d734e46e2cfd2b88fc))

- Set initial release version to 0.1.0 ([#20](https://github.com/akira-io/git-cognition-rs/issues/20)) ([491d77b](akira-io/git-cognition-rs/commit/491d77b7e5a19261c0201d70e273895eeee913d9))


## [workspace-v0.2.0] - 2026-05-25


### Bug Fixes

- Point git-cliff github remote at issue-providers-rs ([6c1da81](akira-io/git-cognition-rs/commit/6c1da81c7e0b4cdc7279c683d36cd57ca4b13e02))

- Publish with --allow-dirty after set-version ([32f9407](akira-io/git-cognition-rs/commit/32f940738cf63b2725b977c89c378f7ce00adb4d))



### Continuous Integration

- Independent per-crate tag-driven versioning ([8715d24](akira-io/git-cognition-rs/commit/8715d24248980d7c3d6e049162697bf2326d5c10))

- Add workspace-wide release tag and manual bump workflow ([01234d9](akira-io/git-cognition-rs/commit/01234d9163649c42b4af08f468b1b12d3f214467))

- Per-crate changelogs and Discord notifications ([9a6d7bc](akira-io/git-cognition-rs/commit/9a6d7bc4b51c9dbd35bbc7604411b9deefe8d732))



### Features

- Enrich Issue with identifier, description, url, author, team, labels, timestamps ([7f0f60b](akira-io/git-cognition-rs/commit/7f0f60b15daab1b467409edffa6a8839c3949ab7))

- Add Viewer capability for authenticated account ([2317cbf](akira-io/git-cognition-rs/commit/2317cbf332ff5c065a97dfb1a2615be5582167e9))

- Add Comments capability ([9f54ffa](akira-io/git-cognition-rs/commit/9f54ffa3e79f73f3689e8e3b2e04454fed509867))

- Filter on Issues::list ([a70ad38](akira-io/git-cognition-rs/commit/a70ad38c934ac2593ad040650423861a3aaad8cf))



### Maintenance

- V0.1.0 ([89c1330](akira-io/git-cognition-rs/commit/89c13308889a22675b065926ddce890103afd680))

- Workspace-v0.2.0 ([cc9d635](akira-io/git-cognition-rs/commit/cc9d635d57007109bc368ed9f4f477e2584ba29c))


## [0.1.0] - 2026-05-25


### Code Refactoring

- Issue() builder accepts raw string ids ([28cb5c9](akira-io/git-cognition-rs/commit/28cb5c953d50be223f636fca5bc14f5688cd7ba0))



### Documentation

- Note builder ids accept string or newtype ([fd5cd34](akira-io/git-cognition-rs/commit/fd5cd34d7b73d58c96890e3bdd45a3bb9bda760f))



### Features

- Scaffold issue-providers-rs with core and linear driver ([8a01a11](akira-io/git-cognition-rs/commit/8a01a114415dbbf7c10d8d60fbedd87f552c5d51))

- Add normalized StatusCategory to Issue ([498b963](akira-io/git-cognition-rs/commit/498b9638492221e09e16f2acd8201e69c8a40f1e))

- Builder id setters accept string or newtype ([5171a3c](akira-io/git-cognition-rs/commit/5171a3c31c113d34d245f281b9fe6869afbb6121))

- Graphql transport and Issues capability ([9da6ee1](akira-io/git-cognition-rs/commit/9da6ee155a79d55c9c77aff06d0a04960d89a9e1))

- Implement named-entity capabilities ([1b9d298](akira-io/git-cognition-rs/commit/1b9d298d3905cfd65ab8f1ff92b59321ebf116ce))

- Add issue mutation verbs (create/update/delete/close) ([f2c9ec3](akira-io/git-cognition-rs/commit/f2c9ec3eb0b034eaeddc9733a236b3cfee803e66))

- Fluent executing terminals on draft/patch builders ([20c895e](akira-io/git-cognition-rs/commit/20c895e16cb80b9d3529337a97e5aa1eceaff3f0))



### Maintenance

- Publish-ready metadata, readmes, and credentialed example ([324330e](akira-io/git-cognition-rs/commit/324330ea0acdafba4af4698e459ebdc20c809c4d))



### Testing

- Add serde round-trip coverage for models and StatusCategory ([ba16e8e](akira-io/git-cognition-rs/commit/ba16e8e6a72740983885173c56b8ff7e5ad1f28d))



