# `canonical-cloud` repository relationships

Generated from reviewed policy and the current **public** repository inventory.

- Public repositories declared: **9**
- Private repository names withheld: **5**
- Relationship edges: **21**

## Repository roles

| Repository | Role | Lifecycle |
|---|---|---|
| [`.github`](https://github.com/canonical-cloud/.github) | `organization_governance` | `active` |
| [`canonical-interfaces`](https://github.com/canonical-cloud/canonical-interfaces) | `interfaces` | `active` |
| [`canonical-api-server.rs`](https://github.com/canonical-cloud/canonical-api-server.rs) | `api_service` | `active` |
| [`canonical-mcp-server.rs`](https://github.com/canonical-cloud/canonical-mcp-server.rs) | `mcp_server` | `active` |
| [`canonical-web-server.rs`](https://github.com/canonical-cloud/canonical-web-server.rs) | `web_bff` | `active` |
| [`canonical-marketing-site.web`](https://github.com/canonical-cloud/canonical-marketing-site.web) | `site` | `active` |
| [`canonical-monorepo`](https://github.com/canonical-cloud/canonical-monorepo) | `composition_workspace` | `active` |
| [`canonical-lib`](https://github.com/canonical-cloud/canonical-lib) | `uncategorized` | `active` |
| [`canonical.cloud`](https://github.com/canonical-cloud/canonical.cloud) | `uncategorized` | `active` |

## Declared edges

| From | Relationship | To | Status/basis |
|---|---|---|---|
| `canonical-cloud/.github` | `governs` | `canonical-cloud/canonical-api-server.rs` | `inferred` / `role-convention`: organization defaults, safety, and relationship declarations |
| `canonical-cloud/.github` | `governs` | `canonical-cloud/canonical-interfaces` | `inferred` / `role-convention`: organization defaults, safety, and relationship declarations |
| `canonical-cloud/.github` | `governs` | `canonical-cloud/canonical-lib` | `inferred` / `role-convention`: organization defaults, safety, and relationship declarations |
| `canonical-cloud/.github` | `governs` | `canonical-cloud/canonical-marketing-site.web` | `inferred` / `role-convention`: organization defaults, safety, and relationship declarations |
| `canonical-cloud/.github` | `governs` | `canonical-cloud/canonical-mcp-server.rs` | `inferred` / `role-convention`: organization defaults, safety, and relationship declarations |
| `canonical-cloud/.github` | `governs` | `canonical-cloud/canonical-monorepo` | `inferred` / `role-convention`: organization defaults, safety, and relationship declarations |
| `canonical-cloud/.github` | `governs` | `canonical-cloud/canonical-web-server.rs` | `inferred` / `role-convention`: organization defaults, safety, and relationship declarations |
| `canonical-cloud/.github` | `governs` | `canonical-cloud/canonical.cloud` | `inferred` / `role-convention`: organization defaults, safety, and relationship declarations |
| `canonical-cloud/canonical-api-server.rs` | `implements_contracts_from` | `canonical-cloud/canonical-interfaces` | `inferred` / `role-convention`: service boundary implements canonical contracts |
| `canonical-cloud/canonical-mcp-server.rs` | `calls` | `canonical-cloud/canonical-api-server.rs` | `inferred` / `role-convention`: agent tools use the authenticated product API |
| `canonical-cloud/canonical-monorepo` | `composes` | `canonical-cloud/canonical-api-server.rs` | `inferred` / `role-convention`: development workspace and release bill of materials |
| `canonical-cloud/canonical-monorepo` | `composes` | `canonical-cloud/canonical-interfaces` | `inferred` / `role-convention`: development workspace and release bill of materials |
| `canonical-cloud/canonical-monorepo` | `composes` | `canonical-cloud/canonical-lib` | `inferred` / `role-convention`: development workspace and release bill of materials |
| `canonical-cloud/canonical-monorepo` | `composes` | `canonical-cloud/canonical-marketing-site.web` | `inferred` / `role-convention`: development workspace and release bill of materials |
| `canonical-cloud/canonical-monorepo` | `composes` | `canonical-cloud/canonical-mcp-server.rs` | `inferred` / `role-convention`: development workspace and release bill of materials |
| `canonical-cloud/canonical-monorepo` | `composes` | `canonical-cloud/canonical-web-server.rs` | `inferred` / `role-convention`: development workspace and release bill of materials |
| `canonical-cloud/canonical-monorepo` | `composes` | `canonical-cloud/canonical.cloud` | `inferred` / `role-convention`: development workspace and release bill of materials |
| `canonical-cloud/canonical-web-server.rs` | `calls` | `canonical-cloud/canonical-api-server.rs` | `inferred` / `role-convention`: client uses the product service boundary |
| `organization://canonical-cloud` | `deployed_via` | `platform://ORESoftware/k8s-cluster` | `platform-default` / `platform-policy`: immutable artifacts are promoted by digest through GitOps |
| `organization://canonical-cloud` | `uses_transport_library` | `platform://ORESoftware/mcp-rust-libs` | `platform-default` / `platform-policy`: shared MCP transport and protocol hardening |
| `organization://canonical-cloud` | `packaged_via` | `platform://zed-pkg` | `platform-default` / `platform-policy`: Zed resolves artifacts while submodules compose editable source |

## Composition, service, and observability contract

Git submodules compose editable source; Zed packages resolve packages/artifacts; dual-managed commits must match. Production deploys immutable image digests, not runtime source builds. Cross-service access uses APIs/SDKs/events rather than another service database. MCP uses the product API/SDK. Services emit OpenTelemetry traces, bounded metrics, and correlated structured logs.

## Privacy boundary

This public registry deliberately omits private repository names and edges; the count above makes the boundary explicit.
