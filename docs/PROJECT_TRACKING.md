# Linear and GitHub Project tracking

This document defines the organization-wide delivery contract for
`canonical-cloud`. It applies to every active repository and keeps product
planning, implementation evidence, and portfolio reporting linked without
copying the same source of truth into multiple systems.

## Systems of record

- **Linear** owns product intent, priority, sequencing, acceptance criteria,
  cross-repository initiatives, and accountable ownership.
- **GitHub issues** own repository-local implementation discussions when a
  durable code, security, or public record is useful.
- **Pull requests and checks** own the exact source change, review evidence,
  tested commit, and merge state.
- **The canonical-cloud organization GitHub Project** is the delivery
  portfolio. It combines Linear links, issues, pull requests, releases, and
  deployment readiness across repositories.
- **Slack `#canonical-cloud`** is for coordination and alerts, not durable
  requirements or final decisions.

Do not duplicate full requirements across these systems. Link the records and
keep each system authoritative for its own concern.

## Repository ownership

| Repository | Delivery responsibility |
| --- | --- |
| `canonical-monorepo` | exact app pins, all-up integration CI, release boundaries, and GitOps handoff |
| `canonical-api-server.rs` | `api.canonical.plus` REST/WebSocket, persistence, owner isolation, and model orchestration |
| `canonical-web-server.rs` | `app.canonical.plus` HTML/HTMX, sessions, offline sync, and revocation worker |
| `canonical-marketing-site.web` | public marketing and authenticated quote-entry links |
| `canonical-interfaces` | generated wire and database contracts |
| `canonical-lib` | reusable domain validation and context assembly |
| `canonical-clients` | language-specific transport clients |
| `canonical-cli` | operator and customer command-line workflows |
| `canonical-mcp-server.rs` | agent-facing diagnostics and organization operations |
| `.github` | organization governance, templates, shared policy, and relationship metadata |
| `canonical.cloud` | superseded compatibility mirror; no new planning or implementation work |

Deployable application and service repositories are pinned as real Git
submodules under `canonical-monorepo/apps/`. Reusable source dependencies are
Zed packages. The intended domain path is
`canonical-interfaces -> canonical-lib -> CLI/API/web/MCP`, with
`canonical-clients` as the transport layer.

## Linking convention

When a Linear issue exists, include its key in the pull-request body and, when
practical, in the branch name or title. A repository issue should link the
Linear issue instead of duplicating its roadmap discussion. The organization
GitHub Project item should link both records and surface the associated pull
request.

Recommended pull-request section:

```markdown
## Tracking

- Linear: CAN-123
- GitHub issue: #456
- GitHub Project: Canonical Cloud / Quote Platform
```

Omit unavailable rows rather than inventing identifiers.

## Status mapping

| Linear | GitHub Project | Code signal |
| --- | --- | --- |
| Triage / Backlog | Backlog | no implementation branch |
| Ready | Ready | acceptance criteria and owner assigned |
| In Progress | In progress | branch or draft pull request exists |
| In Review | In review | non-draft pull request with required checks running |
| Blocked | Blocked | blocker and next decision recorded |
| Done | Done | merged and, when applicable, deployed or explicitly release-ready |
| Canceled | Canceled | closed with rationale; no silent deletion |

Pull-request state should update the GitHub Project automatically. Linear
priority and product status must not be overwritten solely because a branch or
pull request exists.

## GitHub Project fields

Use organization-level fields consistently:

- Status
- Owning repository
- Workstream
- Linear issue
- Pull request
- Target milestone or release
- Risk / blocker
- Deployment state

Store GitHub Project, Linear workspace, and team identifiers in organization or
repository variables used by automation. Never commit API tokens, project IDs,
webhook secrets, or service-account credentials.

## Promotion and staging

Topology changes that affect submodules, Zed package resolution, release
boundaries, shared auth, or cross-origin routing remain draft until their exact
source heads pass the approved `canonical-cloud-test` staging mirrors. Staging
must not publish packages, releases, DNS, cluster configuration, or production
secrets. Promotion uses the exact tested SHA or image digest.

`canonical.cloud` is not mirrored as active source. Staging tests should prove
that it remains a frozen compatibility mirror and that new work resolves
through `canonical-monorepo` and the real source repositories.

## Review cadence

During the weekly Canonical operating review:

1. reconcile blocked and in-review items between Linear and the GitHub Project;
2. verify every active item has one owning repository and one accountable owner;
3. review cross-repository changes in `canonical-monorepo` only after component
   CI is green and exact component commits are pinned;
4. close stale duplicate issues instead of maintaining parallel task copies;
5. record deployment readiness separately from code merge state.

Automation must be idempotent, preserve manual notes, and fail closed when a
Linear or GitHub Project identifier is missing or ambiguous.
