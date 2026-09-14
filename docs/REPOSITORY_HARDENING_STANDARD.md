# Canonical Cloud repository hardening standard

This is the default engineering baseline for production repositories in `canonical-cloud`. Apply only the controls that make sense for a repository's role; document justified deviations instead of cargo-culting files or dependencies.

## 1. Repository identity and boundary

Every repository should make these facts discoverable from its README or repo-local docs:

- one-sentence purpose;
- public/private data sensitivity;
- upstream contract authorities;
- downstream consumers;
- mutable vs read-only responsibilities;
- production deployment target, if any;
- secrets/config boundary;
- owner/on-call or escalation path where applicable;
- known intentional deviations from org defaults.

A repo that implements a shared interface must name the interface authority rather than duplicating it locally.

## 2. Contracts

Contract repositories and contract-consuming servers must follow these rules:

- authored TypeSpec and Draft 2020-12 JSON Schema remain independent peer authorities when that contract family uses both;
- `ORESoftware/typespec-json-schema-validator` admission has zero unexplained parity findings;
- generated folders are read-only outputs with generation provenance;
- valid and invalid fixtures exercise security boundaries, not only happy paths;
- wire names default to `snake_case` where no compatibility constraint overrides it;
- incoming HTTP headers match case-insensitively while authored ORE extension headers use canonical lowercase `x-ores-*` names;
- route maps/API docs stay synchronized with the same typed source contracts;
- optional Protobuf/WIT/formal-method artifacts are checks/projections, not silent replacement authorities.

## 3. Rust and application code

Default rules for Rust services/CLIs:

- `cargo fmt --check`, `cargo check --locked`, unit/integration tests, and Clippy with an explicit warning policy;
- no shell interpolation for untrusted probe/command inputs;
- bounded file/network inputs and explicit timeouts;
- HTTPS by default; loopback HTTP only for local test/development exceptions;
- redirects are explicit policy, not silently followed for security/readiness probes;
- secrets come from the approved secret/config boundary, never command-line flags or logs;
- stable structured errors without leaking tokens, signed URLs, report/evidence bodies, or customer secrets;
- all mutable tenant operations authorize before persistence;
- provider SDK calls remain behind an adapter/domain boundary when provider portability matters.

## 4. HTTP/API services

Customer and admin services must keep their four-server separation. Applicable HTTP services should use the shared ORE layers rather than rebuild them ad hoc:

- `ores-middleware` for common request/response admission;
- `ores-rate-limit` for bounded abuse controls;
- `shared-auth` for authentication/authorization;
- `ores-redis-lru-cache` only as non-authoritative cache state;
- `ores-otel` for structured, redacted telemetry;
- `api-docs` typed route/RPC documentation;
- `.ores-*.toml` and `.zpkg.toml` declarations for shared runtime packages/config.

Catch-all/fall-through HTTP handling returns an intentional 4xx response and must not expose stack traces or route internals.

## 5. Persistence

- migrations are declarative, reviewable, idempotent where practical, and lock/lease protected;
- schema admission is tested against every supported Postgres provider used by the product;
- runtime readiness checks assert named schema invariants/version, not brittle total table/constraint counts;
- tenant scope is part of every relevant primary/unique/index/RLS design decision;
- immutable report/evidence bodies are not rewritten in place;
- relational indexes and immutable object storage are reconciled after partial failures;
- database credentials are least-privilege per server/plane;
- admin and customer data-plane privileges remain separate.

## 6. Infrastructure

`*-infra`/`canonical-infra` should be modules-first and provider-explicit while preserving deployable environment roots. Applicable controls:

- Terraform/OpenTofu formatting/validation and provider lock files;
- explicit state backend and environment boundaries;
- Cloudflare, Supabase, Neon, GCP, Kubernetes and related provider config live in the infra monorepo rather than scattered provider repos unless an exception is documented;
- secrets are SOPS/age or workload-identity backed; plaintext `.env` files are not committed;
- mutation utilities default to plan/dry-run and require an explicit apply gate for consequential production changes;
- exact resource ownership/identity is verified before treating an already-exists response as idempotent success;
- deployment images are promoted by digest;
- network ingress cannot bypass auth-bearing application services.

## 7. CLI tools

Every CLI command should have:

- typed Clap parsing (or equivalent);
- declared flags-2-env mapping for all supported non-secret flags;
- no credential flags;
- bounded stdin/file sizes;
- deterministic JSON output where used in automation;
- human-readable summary output where operator UX benefits;
- stable exit semantics that distinguish command/input failure from readiness/compliance findings;
- `--help` coverage and regression tests for the command contract;
- offline/local behavior where the task does not intrinsically require a hosted service.

For `canonical-cli`, local readiness/pre-audit generation must work without Cloudflare, Supabase, Neon, or Canonical server credentials. Publishing is the separate authenticated network step.

## 8. CI and supply chain

Where relevant:

- pin GitHub Actions to immutable SHAs;
- use lockfiles and `--locked`/frozen installs;
- audit dependencies with a documented allow/deny process rather than blindly failing forever on known advisory noise;
- verify generated outputs are clean after regeneration;
- run ORES repo-baseline smoke checks;
- preserve exact-head evidence for merge/promotion;
- distinguish an Actions job that ran and failed from a job that never started due to account/billing/spending limits;
- never lower security gates solely to make CI green.

## 9. Testing layers

Repositories should use the smallest sufficient combination of:

1. unit tests for deterministic pure behavior;
2. contract/fixture tests for wire semantics;
3. integration tests against real local dependencies/containers where useful;
4. system/e2e tests in `canonical-e2e` across the four-server topology;
5. provider parity tests for Supabase/Neon, Cloud Run/Kubernetes, and Cloudflare boundaries where the feature depends on them;
6. negative security tests for cross-tenant access, path traversal, malformed digests/timestamps, oversized inputs, missing auth, replay/idempotency, and failure recovery.

A mocked provider test is not proof that a live-provider permission or endpoint works; record which layer a result represents.

## 10. Documentation and exceptions

When a repo intentionally deviates from this standard, document:

- the standard being deviated from;
- why the default is not optimal for this repo;
- the replacement design/control;
- affected consumers;
- revisit condition/date if temporary.

Variation is allowed; undocumented accidental drift is not.
