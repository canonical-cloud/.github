# Readiness platform engineering milestones

Status is evidence-based. A milestone is not complete merely because a PR exists or a workflow status is green; required checks must have actually executed against the intended head commit.

## P0 — converge the contract spine

- [x] multi-tenant readiness workspace contract exists in `canonical-interfaces` with TypeSpec/JSON Schema peer authorities;
- [x] tenant-isolated R2 report storage boundary exists in `canonical-infra`;
- [x] deterministic `canonical.readiness.assessment.v1` pre-audit assessment contract is admitted in `canonical-interfaces` (`#72`);
- [x] fail-closed admitted Contract IR projection bridge exists in `canonical-interfaces` (`#73`);
- [ ] feed admitted readiness Contract IR into the supported generated Rust/Rust-WASM/TypeScript/Python/Go/Dart adapters (`canonical-interfaces#75`);
- [ ] add typed report publication/list/detail/content route contracts and API docs from the shared route authority (`canonical-interfaces#74`);
- [ ] add cross-repo conformance fixtures that are consumed by CLI, API, web/clients, persistence, and e2e (`canonical-e2e#14`).

Exit gate: CLI/API/web/ORM cannot disagree on report/workspace shape without contract CI failing.

## P1 — canonical-cli pre-audit/report UX

- [x] bounded single/multi-endpoint readiness probe/scan exists;
- [x] company manifest validation and multi-framework planning exists;
- [ ] promote the draft scan/evidence -> admitted assessment implementation in `canonical-cli#30` only after generated shared types, typed routes, and stepful Rust CI converge;
- [ ] deterministic, finding-linked remediation suggestions with priority and verification steps;
- [ ] report validation with assurance-boundary and content-digest verification;
- [ ] authenticated publish command with environment-only token and HTTPS-only remote endpoint policy;
- [ ] machine-readable + operator summary output;
- [ ] regression coverage for input limits, unknown evidence, scan errors, report determinism, auth token absence, unsafe endpoints, and API reject paths;
- [ ] flags-2-env parity tests for every new command/flag, including an explicit decision for the current Clap-only `preaudit` alias;
- [ ] remove duplicate/dead dispatch paths and enforce Clippy warning policy.

Exit gate: a local customer can run scope → checks → pre-audit report without cloud-provider credentials; publication is a separate explicit step.

## P2 — API publication and storage transaction

- [ ] admit and implement readiness report publication/list/detail/content under the canonical shared `/api/v1/readiness/...` route namespace; exact paths and types belong to `canonical-interfaces#74`, not a client or service-local string;
- [ ] tenant/scope always derived/validated from shared-auth context;
- [ ] request size limits, rate limiting, middleware, and typed contract validation occur before mutation;
- [ ] server recomputes content digest and assessment identity;
- [ ] immutable R2 object create uses tenant-owned bucket/key;
- [ ] read-after-write digest verification before index commit;
- [ ] `ReportIndexRecord` persistence through `canonical-orm-core`;
- [ ] exact idempotency semantics for duplicate publication (`canonical-api-server.rs#55`);
- [ ] orphaned R2 object reconciliation after relational failure (`canonical-api-server.rs#56`);
- [ ] no report body/signed URL/token leakage to telemetry;
- [ ] merge named schema readiness invariants after stepful application compile/test evidence (`canonical-api-server.rs#54`).

Exit gate: cross-tenant, wrong-digest, replay, partial-failure and oversized-input integration tests pass.

## P3 — web/customer workspace

- [ ] tenant-scoped report list/search/detail/content views;
- [ ] user and organization membership/role tests;
- [ ] recommendations separated from immutable report content;
- [ ] checklist/evidence expiry/risk/incident/training/form workflows wired to readiness-workspace contracts;
- [ ] authorized report content flows only through the customer API boundary; no browser-visible R2 authority (`canonical-web-server.rs#141`);
- [ ] no direct privileged R2/DB mutation from customer web rendering paths;
- [ ] accessibility, empty/error/loading states, and pagination/search tests;
- [ ] Flutter/client parity for supported workspace/report operations.

Exit gate: `canonical-e2e` proves user A cannot enumerate, fetch, mutate, infer, or publish into tenant B.

## P4 — Supabase + Neon persistence parity

- [ ] identical admitted migrations/schema invariants pass on Supabase Postgres and Neon Postgres;
- [ ] one authoritative writer declared per logical dataset/environment;
- [ ] standby/replica/projection behavior documented;
- [ ] write-authority fencing/promotion mechanism implemented (`canonical-orm-core#9`);
- [ ] migration locks/leases use approved shared implementation;
- [ ] failover and reconciliation tests cover report indexes and remediation records;
- [ ] RLS/role grants and application authorization are tested independently.

Exit gate: controlled failover does not create two accepted conflicting writers or cross-tenant visibility.

## P5 — Cloud Run + Kubernetes deployment parity

- [ ] same immutable image digest deployable to GCP Cloud Run and `ORESoftware/k8s-cluster`, with executable digest comparison (`canonical-infra#45`);
- [ ] flags-2-env/SOPS-age config parity;
- [ ] workload identity/short-lived credentials where available;
- [ ] graceful shutdown/timeouts/resource limits tested;
- [ ] `/healthz` and `/readyz` semantic parity;
- [ ] OpenTelemetry redaction and trace correlation parity;
- [ ] Cloudflare ingress failover to healthy backend without auth bypass;
- [ ] disaster recovery runbook covers app services, relational index, R2 content, writer promotion/fencing and reconciliation (`canonical-docs#26`).

Exit gate: an e2e report publish/read flow passes against both scheduler targets using the same application artifact.

## P6 — continuous readiness/evidence program

- [x] signed continuous-readiness observation primitives exist;
- [ ] move observation ingress onto one typed shared route authority and explicitly resolve compatibility aliases, header namespace, and server-derived tenant/source authority;
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

1. **GitHub Actions admission and private dependency access:** distinguish zero-step runner/account failures from executed tests. Separately, the monorepo and API currently reproduce a private Cargo dependency-auth blocker before API tests (`canonical-monorepo#94`). Neither state is source-test evidence.
2. **Generated consumer + route gap:** the assessment contract and admitted-IR bridge are merged, but generated consumer adapters (`canonical-interfaces#75`) and typed report routes (`canonical-interfaces#74`) must land before service/client-local structs or URL strings can stabilize.
3. **API storage transaction:** R2 boundary exists, but authenticated report ingest/list/detail/content and relational transaction/reconciliation must converge across API/ORM/infra/e2e (`canonical-api-server.rs#52/#55/#56`, `canonical-orm-core#7/#9`, `canonical-infra#43`).
4. **API readiness compile evidence:** named schema invariants exist in draft `canonical-api-server.rs#54` and the PostgreSQL declarative lane is green, but the application Rust lane has not executed past its private-library credential gate.
5. **Documentation authority drift:** current docs disagree on Canonical Cloud vs Canonical Plus host/brand roles, route aliases, repository inventory, and some legacy source-of-truth wording. Resolve explicit policy decisions first, then synchronize all consumers; do not use documentation drift to create a second executable authority.
