# Canonical test-org CI policy

Effective 2026-08-07, `canonical-cloud` is the source/product organization and must not be used for routine GitHub-hosted Actions while its included Actions minutes are exhausted.

## Execution organization

Use `canonical-cloud-test` for CI, integration, E2E, package-consumer, cross-repository, and exact-head staging runs. Production repositories remain authoritative for reviewed source; test repositories consume reviewed immutable source heads and must not become release authorities.

## Required safety boundaries

- Do not bypass required checks in `canonical-cloud` because hosted Actions are unavailable.
- Keep blocked product PRs draft until their exact reviewed heads have equivalent executable evidence from the isolated test organization.
- Test workflows are no-secrets/no-publish by default. They must not deploy production workloads, mutate production Cloudflare/R2/Kubernetes resources, or publish production artifacts.
- Mirror source by immutable commit SHA or reviewed gitlink/package lock. Never silently substitute a newer branch tip.
- Keep the test topology idempotent and reuse existing repositories/branches/runs instead of creating duplicates.
- Record source repository, PR, exact head SHA, test repository, workflow/run evidence, and result in Linear/GitHub tracking before promotion.

## Initial staging matrix

The reviewed `canonical-e2e` topology defines these intended isolated repositories:

- `canonical-api-server.rs`
- `api-server-contract-e2e`
- `monorepo-submodules-e2e`
- `zed-package-graph-e2e`
- `web-server-routing-e2e`
- `cli-install-e2e`
- `clients-rust-consumer`
- `clients-typescript-consumer`
- `clients-go-consumer`
- `clients-python-consumer`
- `mcp-contract-e2e`
- `legacy-mirror-guard-e2e`

## Current provisioning blocker

At the time this policy was recorded, `canonical-cloud-test` was not visible to the connected GitHub App. Provisioning and execution therefore remain blocked until the organization exists and the app/acting credential has the required repository and Actions permissions there.

Once access is available, prioritize exact-head staging for the currently blocked Canonical PRs, including `canonical-monorepo#32`, `canonical-web-server.rs#40`, `canonical-flutter#1`, `canonical-clients#20`, `canonical-lib#2`, and their current successors where applicable.

## Promotion rule

A production PR may advance only when the evidence corresponds to its exact reviewed source head (or a documented successor containing only reviewed corrective changes), the intended tests actually executed, and the staging result is linked into the Canonical Linear project and GitHub tracking issue/project.
