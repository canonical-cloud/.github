# Canonical Cloud repository architecture

This document is the organization-level ownership map for the Canonical Cloud repository fleet. It is intentionally explicit so that `canonical-cli`, the customer web/API surfaces, persistence, infrastructure, and test repositories evolve without collapsing into one deployment-specific implementation.

## Architectural rules

1. `canonical-cli` is the customer/operator audit producer. It may execute bounded local probes, generate deterministic reports, validate them, fetch audit programs, and submit reports. It must not contain Supabase-, Neon-, Cloudflare-, GCP-, or Kubernetes-specific persistence logic.
2. `canonical-interfaces` owns cross-runtime wire contracts. Human-authored TypeSpec and human-authored JSON Schema Draft 2020-12 are independent peer authorities. Generated JSON Schema is comparison evidence only. Cross-runtime report/account contracts must pass `ORESoftware/typespec-json-schema-validator` admission.
3. `canonical-clients` owns generated/hand-written clients for the public contracts. `canonical-lib` and `canonical-lib-core` provide reusable public/private implementation logic; `canonical-orm-core` owns private persistence mappings and migrations, not public API shapes.
4. `canonical-api-server.rs` owns customer write APIs and report persistence. `canonical-web-server.rs` owns customer read/render flows. Admin write/read paths remain in the separate `canonical-admin-api-server.rs` and `canonical-admin-web-server.rs` security planes.
5. Report persistence is Postgres-compatible and storage-neutral. Supabase and Neon are supported deployment authorities behind the same SQL/RLS contract. The application must not expose which provider served a request.
6. `canonical-infra` owns Cloudflare, GCP Cloud Run, Kubernetes, Supabase, Neon, DNS, routing, secrets references, service identity, and deployment composition. Provider-specific repositories must not reappear as parallel authorities.
7. `canonical-e2e` owns cross-service acceptance. A feature is not fleet-complete until the test repository proves the CLI -> API -> database -> web retrieval path and validates tenant isolation.
8. `canonical-company-auditor.rs` may orchestrate company-scale evidence collection, but it consumes the same report/account contracts as `canonical-cli`; it is not a second report format authority.
9. Secrets are environment/secret-store only. Bearer tokens, database credentials, signing keys, and provider credentials must never be accepted on argv or serialized into audit reports.
10. Audit/readiness output is evidence, not an audit opinion, certification, attestation, authorization, or guarantee of compliance.

## Customer audit report flow

The canonical customer flow is:

```text
AuditProgram
   |
   v
canonical-cli / canonical-company-auditor.rs
   |
   | CanonicalAuditReportV1 + sha256 digest
   v
canonical-api-server.rs
   |
   +--> authenticate subject with shared-auth
   +--> authorize user/org account scope
   +--> validate report contract and immutable digest
   +--> persist report envelope + report JSON in Postgres
   +--> append report event / receipt
   |
   v
Supabase PostgreSQL or Neon PostgreSQL
   |
   v
canonical-web-server.rs
   |
   +--> list customer reports
   +--> render report summary/details/evidence
   +--> expose stable deep links scoped to user/org account
```

Cloudflare may terminate/proxy edge traffic. GCP Cloud Run and `ORESoftware/k8s-cluster` are deployment targets for the same server binaries. None of those deployment choices alter the report wire contract.

## Report account model

Reports belong to exactly one account scope:

- `user`: the authenticated user's personal account;
- `org`: an organization account for which the authenticated subject has an allowed membership/role.

The API, not the CLI, is authoritative for account membership. The CLI sends the requested owner kind/id; the server resolves and authorizes it against shared-auth before storage.

Minimum immutable report envelope fields:

- schema version;
- report ID (UUID or other bounded canonical identifier assigned by the server);
- `ownerKind` and `ownerId`;
- authenticated `subjectId` that submitted the report;
- `programId`, `programVersion`, and program digest;
- report digest over the canonical report JSON;
- report creation timestamp and server ingestion timestamp;
- source (`canonical-cli`, `canonical-company-auditor`, CI, or other admitted source);
- report JSON document;
- optional labels/tags that do not participate in report identity.

Updates to mutable presentation metadata must never rewrite the immutable report JSON or its digest. Deletion/retention must be explicit lifecycle state, not silent row replacement.

## Repository ownership map

| Repository | Primary responsibility |
| --- | --- |
| `.github` | organization governance, fleet policies, architecture and shared workflow policy |
| `canonical-cli` | local audit execution, validation, report generation/submission, framework/company commands |
| `canonical-company-auditor.rs` | company-scale audit/evidence orchestration using canonical contracts |
| `canonical-interfaces` | TypeSpec + authored JSON Schema peer authorities and generated language interfaces |
| `canonical-clients` | API/client bindings for Canonical contracts |
| `canonical-lib` | public reusable library surface |
| `canonical-lib-core` | private/shared implementation core |
| `canonical-orm-core` | private ORM entities, migrations, SQL/RLS implementation |
| `canonical-api-server.rs` | customer write API, authz, report ingestion/persistence, receipts |
| `canonical-web-server.rs` | customer read/render experience, report browsing and deep links |
| `canonical-admin-api-server.rs` | isolated admin write/API plane |
| `canonical-admin-web-server.rs` | isolated admin UI/read plane |
| `canonical-mcp-server.rs` | non-admin MCP surface over admitted customer capabilities |
| `canonical-sidecar.rs` | bounded sidecar/runtime integration |
| `canonical-lambdas` | event-driven/background serverless handlers |
| `canonical-sync` | cross-device/service synchronization where required |
| `canonical-e2e` | cross-service, provider, tenancy, report lifecycle and deployment acceptance |
| `canonical-infra` | all provider IaC: Cloudflare, GCP, Kubernetes, Supabase, Neon and routing |
| `canonical-monorepo` | integration/workspace aggregation, not an independent contract authority |
| `canonical-flutter` | Flutter client/application surface |
| `canonical-desktop-app.rs` | Rust-native desktop application surface |
| `canonical.cloud` | primary customer-facing site/application repository |
| `canonical-marketing-site.web` | marketing web surface |
| `canonical-cloud.github.io` | GitHub Pages/public static surface |
| `canonical-docs` | product, legal, security, readiness and operator documentation |
| `canonical-assets` | shared brand/static assets |

## Fleet hardening baseline

Every active repository should be evaluated against the following baseline, with documented exceptions in this repository:

- canonical `AGENTS.md` pointer and `.ores/` ignore;
- `tmp/`, `temp/`, and repository-local worktrees ignored;
- exact/pinned CI action provenance where practical, credentials disabled for read-only PR checkouts, explicit permissions and bounded timeouts;
- patch-exact Rust authority for Rust production repositories, with imported authority explicitly verified when the compiler is owned by another checked-out repository;
- no moving `stable`/`beta`/`nightly` compiler claims in required production CI;
- `.zpkg.toml` / `.zpkg.lock` where the repository participates in Zed package relationships;
- flags-2-env contract parity for CLIs/servers with command-line surfaces;
- SOPS + age / secret-store configuration with no plaintext production `.env` authority;
- ORES structured logging/telemetry and sensitive-field redaction;
- TypeSpec + authored JSON Schema + TJSV admission for cross-runtime contracts;
- generated trees treated as read-only evidence/output;
- SQL/RLS/migrations owned by ORM/infra/server layers rather than duplicated in clients;
- exact user/org tenant isolation tests for customer data;
- `canonical-e2e` acceptance when a change crosses repository boundaries.

## Report-platform completion criteria

The report platform is considered complete only when all of the following are green on exact reviewed heads:

1. `canonical-interfaces`: report envelope/account contracts pass TJSV with positive and negative corpus fixtures.
2. `canonical-cli`: report generation validates the canonical report, supports stdin/stdout streaming, and can submit without writing plaintext temporary files.
3. `canonical-api-server.rs`: authenticated report create/list/get endpoints enforce owner scope, digest integrity, request bounds, RLS, and least privilege.
4. `canonical-web-server.rs`: authenticated report list/detail pages never accept owner identity from an untrusted query alone; the API determines authorization.
5. `canonical-orm-core`: migrations create immutable report storage and tenant-safe indexes/RLS policies.
6. `canonical-infra`: both Supabase and Neon database targets plus Cloud Run/k8s deployment paths are described/tested without becoming API-contract authorities.
7. `canonical-e2e`: a user report and org report can be submitted and retrieved, cross-tenant reads fail closed, tampered digests are rejected, and provider/deployment choice does not change behavior.
