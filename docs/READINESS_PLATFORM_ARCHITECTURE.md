# Readiness platform cross-repo architecture

This document is the engineering map for Canonical Cloud's readiness platform. It complements company/product documentation in `canonical-docs` and contract documentation in `canonical-interfaces`.

## Architectural rule

Portable readiness semantics belong in shared contracts. Provider adapters, deployment topology, database products, and object-storage details belong in infrastructure/persistence implementations. A repo may deviate from the standard shape only when the exception and its replacement control are documented.

## Repository responsibility map

| Repository | Responsibility | Must not own |
| --- | --- | --- |
| `canonical-interfaces` | TypeSpec + authored JSON Schema peer authorities, valid/invalid fixtures, generated language/route projections | provider credentials, runtime tenant authorization |
| `canonical-cli` | local scope/planning, bounded checks, pre-audit orchestration, contract validation, authenticated publication client | R2 credentials, database credentials, tenant authorization decisions |
| `canonical-company-auditor.rs` | company-wide aggregation, requirement-level programs, assessor/reviewer workflow | generic storage schema duplicated from interfaces |
| `canonical-api-server.rs` | mutable authenticated customer API, tenant/scope authorization, report publication orchestration | HTML rendering authority, admin-plane shortcuts |
| `canonical-web-server.rs` | customer-facing HTML/read workflows and authorized report presentation | direct privileged database/R2 mutation paths |
| `canonical-admin-api-server.rs` / `canonical-admin-web-server.rs` | privileged admin workflows in isolated plane | customer-plane authentication shortcuts |
| `canonical-orm-core` | provider-portable persistence implementation and migrations | customer wire-contract authority |
| `canonical-lib-core` | reusable domain implementation behind public/shared interfaces | repo-local competing contracts |
| `canonical-infra` | Cloudflare, Supabase, Neon, GCP/Cloud Run, Kubernetes deployment/provider configuration | readiness business semantics |
| `canonical-e2e` | four-server and provider-boundary integration/system tests | production secrets or mutable production fixtures |
| `canonical-docs` | company/product/customer documentation | executable policy that belongs in `.github`/CI |
| `.github` | cross-repo policy, coding/infra standards, planning, milestones, exception registry | product wire schemas |

## Contract dependency direction

```text
canonical-interfaces
       |
       +--> canonical-cli
       +--> canonical-clients / canonical-flutter
       +--> canonical-lib / canonical-lib-core
       +--> canonical-orm-core
       +--> canonical-api-server.rs
       +--> canonical-web-server.rs
       +--> admin servers
       +--> canonical-company-auditor.rs
       +--> canonical-e2e

canonical-infra consumes service/runtime contracts; it does not become their authority.
```

Generated files remain generated/read-only. Consumers should depend on admitted generated projections or Contract IR, not edit them to resolve drift.

## Report publication boundary

The existing readiness-workspace contract separates deterministic assessment identity from immutable publication identity. Preserve that split:

1. CLI/auditor produces an admitted assessment body and content digest.
2. API authenticates and derives tenant/scope authorization.
3. API validates the shared contract and recomputes the digest.
4. API creates the publication UUID and tenant-owned R2 key.
5. API writes immutable content to the tenant R2 bucket.
6. API reads/verifies the stored digest.
7. API commits `ReportIndexRecord` and any separately mutable recommendation records.
8. Web reads the relational index first, then obtains report content only after authorization.

No customer CLI receives R2 credentials. No user-controlled tenant parameter alone authorizes access. Generic object-store conflicts do not establish idempotent success without exact authenticated object/digest verification.

## Database authority

Supabase Postgres and Neon Postgres are both supported. They are not independent active-active authorities by default.

For every logical dataset, document:

- current authoritative writer;
- standby/replica/projection role, if any;
- migration authority and schema version;
- promotion/fencing epoch;
- reconciliation procedure;
- rollback path;
- RPO/RTO assumptions.

`canonical-orm-core` should keep SQL/business semantics portable. `opto-sync` may synchronize allowed projections but does not grant an offline or secondary replica write authority that bypasses server authorization.

## Hosting topology

Customer web/API and admin web/API remain four separate processes. Production scheduling may use GCP Cloud Run, `ORESoftware/k8s-cluster`, or both, but the runtime contract is the same:

- immutable container image digests;
- SOPS/age-backed secret injection;
- flags-2-env configuration contract;
- provider-neutral `/healthz` and `/readyz`;
- graceful shutdown and bounded timeouts;
- `ores-otel` telemetry with sensitive payload redaction;
- approved `ores-middleware`, `ores-rate-limit`, shared-auth, and cache boundaries;
- no local-disk persistence assumption.

Cloudflare is allowed to terminate/route edge traffic and host R2 artifacts, but it must not bypass application authentication/authorization for tenant data.

## Readiness of the readiness platform

`/readyz` checks should assert named invariants, not brittle raw object counts. Examples:

- required schema/migration version admitted;
- required tables/columns/indexes/policies exist by name;
- database writer can perform the bounded operation the service needs;
- R2 configuration references the expected tenant namespace policy;
- shared-auth dependency/config is available;
- required secrets are present without disclosing them.

Hard-coded counts such as “exactly N relations/constraints” are fragile whenever an admitted schema evolves and should be replaced with named invariant checks.

## Change admission

A cross-repo feature is not complete until the relevant layers converge:

1. contracts/fixtures;
2. implementation;
3. generated clients/interfaces where applicable;
4. persistence/migrations;
5. provider/deployment config;
6. e2e/system tests;
7. operator/company documentation;
8. exact-head CI evidence.

Where Actions are blocked before jobs start, that state is an infrastructure/account blocker, not passing or failing code evidence. Merge/promotion decisions must distinguish zero-step workflow failures from executed tests.
