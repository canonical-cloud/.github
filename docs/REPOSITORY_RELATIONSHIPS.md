<!-- ore-org-baseline:begin -->
# Repository relationships for `canonical-cloud`

This file is rendered from `repository-relationships.json`. The JSON registry is authoritative.

- Audience: `public`
- Repositories represented: **7**
- Relationships represented: **8**
- Inventory digest: `sha256:8e76d3281c8316233b2daa300e42f3419ffd5665bb1ab275b17beec3c9149e5d`

## Immutable routing identity

| Field | Value |
|---|---|
| Mapping ID | `context:canonical-cloud` |
| GitHub owner ID | `297250874` |
| Linear project ID | `9b3cec23-bc6f-4f8d-a696-2bc8a918e86a` |
| Linear team ID | `eb8ab169-5afe-4b6f-9cab-3f2aa3e887dc` |

## Repositories

| Repository | Visibility | Roles | Archived |
|---|---|---|---|
| `canonical-cloud/.github` | `public` | `community-health`, `governance`, `relationship-registry` | no |
| `canonical-cloud/canonical-interfaces` | `public` | `interfaces` | no |
| `canonical-cloud/canonical-marketing-site.web` | `public` | `repository` | no |
| `canonical-cloud/canonical-mcp-server.rs` | `public` | `mcp-server` | no |
| `canonical-cloud/canonical-monorepo` | `public` | `monorepo` | no |
| `canonical-cloud/canonical-web-server.rs` | `public` | `web-server` | no |
| `canonical-cloud/canonical.cloud` | `public` | `repository` | no |

## Relationships

| From | Type | To | Status | Required |
|---|---|---|---|---|
| `canonical-cloud/.github` | `governs` | `canonical-cloud/canonical-interfaces` | `declared` | yes |
| `canonical-cloud/.github` | `governs` | `canonical-cloud/canonical-marketing-site.web` | `declared` | yes |
| `canonical-cloud/.github` | `governs` | `canonical-cloud/canonical-mcp-server.rs` | `declared` | yes |
| `canonical-cloud/.github` | `governs` | `canonical-cloud/canonical-monorepo` | `declared` | yes |
| `canonical-cloud/.github` | `governs` | `canonical-cloud/canonical-web-server.rs` | `declared` | yes |
| `canonical-cloud/.github` | `governs` | `canonical-cloud/canonical.cloud` | `declared` | yes |
| `canonical-cloud/canonical-mcp-server.rs` | `depends_on` | `canonical-cloud/canonical-interfaces` | `inferred` | no |
| `canonical-cloud/canonical-web-server.rs` | `depends_on` | `canonical-cloud/canonical-interfaces` | `inferred` | no |

## Editing relationships

Put reviewed public declarations in `repository-relationships.manual.json`; do not edit the generated registry directly.
Private repository names and private-only relationships belong in the private `approved-private-registry` mirror.
Inferred edges are advisory and must remain visibly labeled until reviewed.
<!-- ore-org-baseline:end -->
