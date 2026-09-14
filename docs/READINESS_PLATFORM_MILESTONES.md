# Readiness platform engineering milestones

Status is evidence-based. A milestone is not complete merely because a PR exists or a workflow status is green; required checks must have actually executed against the intended head commit.

## P0 — converge the contract spine

- [x] multi-tenant readiness workspace contract exists in `canonical-interfaces` with TypeSpec/JSON Schema peer authorities;
- [x] tenant-isolated R2 report storage boundary exists in `canonical-infra`;
- [ ] add/admit a deterministic pre-audit assessment-result contract if no existing admitted contract already covers the CLI/auditor result body;
- [ ] generate/consume Rust, TypeScript, Dart and other needed projections instead of repo-local report structs;
- [ ] add typed report publication/list/detail route contracts and API docs;
- [ ] add cross-repo conformance fixtures that are consumed by CLI, API, web/clients, persistence, and e2e.

Exit gate: CLI/API/web/ORM cannot disagree on report/workspace shape without contract CI failing.

## P1 — canonical-cli pre-audit/report UX

- [x] bounded single/multi-endpoint readiness probe/scan exists;
- [x] company manifest validation and multi-framework planning exists;
- [ ] compose scan + authorized evidence into admitted assessment output;
- [ ] deterministic, finding-linked remediation suggestions with priority and verification steps;
- [ ] report validation with assurance-boundary and content-digest verification;
- [ ] authenticated publish command with environment-only token and HTTPS-only remote endpoint policy;
- [ ] machine-readable + operator summary output;
- [ ] regression coverage for input limits, unknown evidence, scan errors, report determinism, auth token absence, unsafe endpoints, and API reject paths;
- [ ] flags-2-env parity tests for every new command/flag;
- [ ] remove duplicate/dead dispatch paths and enforce Clippy warning policy.

Exit gate: a local customer can run scope → checks → pre-audit report without cloud-provider credentials; publication is a separate explicit step.

## P2 — API publication and storage transaction

- [ ] `POST /v1/readiness/reports` admitted and implemented;
- [ ] tenant/scope always derived/validated from shared-auth context;
- [ ] request size limits, rate limiting, middleware, and typed contract validation occur before mutation;
- [ ] server recomputes content digest and assessment identity;
- [ ] immutable R2 object create uses tenant-owned bucket/key;
- [ ] read-after-write digest verification before index commit;
- [ ] `ReportIndexRecord` persistence through `canonical-orm-core`;
- [ ] exact idempotency semantics for duplicate publication;
- [ ] orphaned R2 object reconciliation after relational failure;
- [ ] no report body/signed URL/token leakage to telemetry;
- [ ] readiness checks use named schema invariants rather than hard-coded total relation/constraint counts.

Exit gate: cross-tenant, wrong-digest, replay, partial-failure and oversized-input integration tests pass.

## P3 — web/customer workspace

- [ ] tenant-scoped report list/search/detail/content views;
- [ ] user and organization membership/role tests;
- [ ] recommendations separated from immutable report content;
- [ ] checklist/evidence expiry/risk/incident/training/form workflows wired to readiness-workspace contracts;
- [ ] short-lived authorized content access where object streaming is not used;
- [ ] no direct privileged R2/DB mutation from customer web rendering paths;
- [ ] accessibility, empty/error/loading states, and pagination/search tests;
- [ ] Flutter/client parity for supported workspace/report operations.

Exit gate: `canonical-e2e` proves user A cannot enumerate, fetch, mutate, infer, or publish into tenant B.

## P4 — Supabase + Neon persistence parity

- [ ] identical admitted migrations/schema invariants pass on Supabase Postgres and Neon Postgres;
- [ ] one authoritative writer declared per logical dataset/environment;
- [ ] standby/replica/projection behavior documented;
- [ ] write-authority fencing/promotion mechanism implemented;
- [ ] migration locks/leases use approved shared implementation;
- [ ] failover and reconciliation tests cover report indexes and remediation records;
- [ ] RLS/role grants and application authorization are tested independently.

Exit gate: controlled failover does not create two accepted conflicting writers or cross-tenant visibility.

## P5 — Cloud Run + Kubernetes deployment parity

- [ ] same immutable image digest deployable to GCP Cloud Run and `ORESoftware/k8s-cluster`;
- [ ] flags-2-env/SOPS-age config parity;
- [ ] workload identity/short-lived credentials where available;
- [ ] graceful shutdown/timeouts/resource limits tested;
- [ ] `/healthz` and `/readyz` semantic parity;
- [ ] OpenTelemetry redaction and trace correlation parity;
- [ ] Cloudflare ingress failover to healthy backend without auth bypass;
- [ ] disaster recovery runbook covers app services, relational index, and R2 content.

Exit gate: an e2e report publish/read flow passes against both scheduler targets using the same application artifact.

## P6 — continuous readiness/evidence program

- [x] signed continuous-readiness observation primitives exist;
- [ ] replay/freshness/idempotency end-to-end coverage;
- [ ] least-privilege evidence connectors with bounded provenance;
- [ ] company auditor requirement-level program ingestion and reviewer states;
- [ ] evidence expiry/renewal automation;
- [ ] historical report comparison and regression detection;
- [ ] framework crosswalk tests proving no cross-framework state inheritance;
- [ ] scheduled/continuous checks remain non-destructive by default.

Exit gate: recurring readiness can update workflow state while every published source report remains immutable and independently attributable.

## Fleet waves

### Wave A — contract/runtime spine
`canonical-interfaces`, `canonical-cli`, `canonical-company-auditor.rs`, `canonical-lib`, `canonical-lib-core`, `canonical-orm-core`, `canonical-clients`.

### Wave B — customer/admin services
`canonical-api-server.rs`, `canonical-web-server.rs`, `canonical-admin-api-server.rs`, `canonical-admin-web-server.rs`, `canonical-sidecar.rs`, `canonical-lambdas`.

### Wave C — infra and testing
`canonical-infra`, `canonical-e2e`, `canonical-sync`, desktop/Flutter clients, marketing/public sites where they exercise authenticated flows.

### Wave D — docs/governance
`canonical-docs`, `.github`, public documentation sites, operator runbooks, incident/recovery docs.

Each wave applies `REPOSITORY_HARDENING_STANDARD.md` selectively and records exceptions. Do not mechanically add service-only dependencies to libraries, docs, or static sites.

## Current blockers to treat explicitly

1. **GitHub Actions zero-step failures:** several existing issues report account payment/spending-limit failures before jobs start. Restore executable CI and exact-head evidence; do not classify zero-step failures as code failures or passing validation.
2. **Contract gap:** the readiness workspace defines the publication envelope/index/workflow records, but the CLI pre-audit assessment body must be mapped to an admitted shared contract before stabilization.
3. **API storage transaction:** R2 boundary exists, but authenticated report ingest/list/detail and relational transaction/reconciliation must converge across API/ORM/infra/e2e.
4. **Brittle API readiness checks:** schema readiness must move from raw count assertions to named/versioned invariants so additive admitted migrations do not break readiness incorrectly.
