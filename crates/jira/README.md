# issue-provider-jira

Jira issue-tracker driver for [`issue-provider-core`](../core), targeting the Jira Cloud REST API v3.

## Capabilities

- Issues (get / list via JQL / create / update / delete / close)
- Projects (get / list)
- Users (get / list)
- Labels (get / list)
- Viewer (current user)
- Comments (list / post / delete)

## Usage

```rust
use issue_provider_jira::jira;

let client = jira()
    .auth("https://your-domain.atlassian.net", "you@example.com", "<api-token>")
    .build();
```

Authentication uses HTTP Basic auth with your Atlassian account email and an
API token. Status changes are applied through Jira issue transitions, and issue
descriptions and comments are read from / written as Atlassian Document Format
(ADF).
