# Canonical Plus authenticated quote execution ledger — August 7, 2026

This ledger reconciles the active Canonical Plus quote work across GitHub, Linear, the Canonical organization Project, Shared Auth, and `ORESoftware/k8s-cluster`.

## Planning authorities

- GitHub organization: [`canonical-cloud`](https://github.com/canonical-cloud)
- GitHub Project: [`canonical-cloud-project`](https://github.com/orgs/canonical-cloud/projects/1)
- Canonical Linear project: [`github.com/canonical-cloud`](https://linear.app/denman/project/githubcomcanonical-cloud-1659c8ea1adf)
- Stable Project routing card: [`.github#2`](https://github.com/canonical-cloud/.github/issues/2)
- Primary Linear delivery issue: `DEN-2622`
- Standalone API and durability issues: `DEN-2598` and `DEN-2599`
- Kubernetes activation issue: `DEN-2601`
- Cloudflare inventory and routing issues: `DEN-2631`, `DEN-2649`, and `DEN-2655`
- Staged service-token compatibility cleanup: `DEN-2825`

GitHub is authoritative for repositories, commits, pull requests, checks, artifacts, and deployment evidence. Linear is authoritative for scope, priority, ownership, dependencies, milestones, and status. The organization Project is the cross-repository execution view.

## Landed source boundaries

| Boundary | Evidence | State |
| --- | --- | --- |
| Shared Auth browser ceremony | [`shared-auth-server.rs#41`](https://github.com/shared-auth/shared-auth-server.rs/pull/41), merge `22aab1de937620251f4e0b9a617c485733c97ff5` | Merged. Realm-specific magic-link/OTP login, sealed relative returns, host-only cookies, refresh, and Canonical overlay. |
| Shared Auth edge behavior | [`shared-auth-infra#9`](https://github.com/shared-auth/shared-auth-infra/pull/9), merge `6234f1ee72349f84652c85a5a957b2982ea471bf` | Merged. Canonical issuer/audience/cookie namespace, refresh verification, header stripping, and 12/12 Node Worker tests. |
| Signed-in web application baseline | [`canonical-web-server.rs#41`](https://github.com/canonical-cloud/canonical-web-server.rs/pull/41), merge `74448c8dcb885fbb240ac59d1079a929bd06caa5` | Merged. Maud/HTMX `/u/quote`, origin authorization, CSRF, forced RLS, REST recovery, WebSocket hints, and configurable model selection. |
| Durable quote API | [`canonical-api-server.rs#6`](https://github.com/canonical-cloud/canonical-api-server.rs/pull/6), merge `91ac093bf6c3d0958918fc8678af95dd13975f1e` | Merged. Durable PostgreSQL queue, leases/recovery, owner-scoped REST/WebSocket API, bounded Gemini analysis, and non-root container. |
| Shared context key | [`canonical-api-server.rs#8`](https://github.com/canonical-cloud/canonical-api-server.rs/pull/8), merge `e3a7cc79b3ceac0e455b9d7822a29d4154c9584b` | Merged. Web and API default to one active `quote-analysis` context record. |
| Gemini 3.6 Pro default | [`canonical-api-server.rs#13`](https://github.com/canonical-cloud/canonical-api-server.rs/pull/13), merge `f57528ccf2b077644917c1f770c97eca3027b8e7` | Merged after exact-head source, test, model-contract, container, and health checks. `GEMINI_MODEL` remains an explicit operator override. |
| Web-to-API source cutover | [`canonical-web-server.rs#49`](https://github.com/canonical-cloud/canonical-web-server.rs/pull/49), merge `dcb979956a247f35a8470280717d0750109f2320` | Merged after exact-head strict Clippy, workspace/client tests, browser E2E, audit, PostgreSQL and CockroachDB RLS, declarative-schema convergence, non-root smoke, and web/API/revoker image contracts. The web retains origin Shared Auth, CSRF, and verified-subject projection; the duplicate web quote/Gemini engine is removed. |
| Canonical edge deployment source | [`canonical-infra#4`](https://github.com/canonical-cloud/canonical-infra/pull/4), merge `03d37469a6ea5ee075a89c064ee60017ae4ebf23` | Merged. Byte-verified reviewed Worker source, route/realm/cookie contracts, and provenance lock. No live Cloudflare write. |
| Quote-v1 wire authority | [`canonical-interfaces#27`](https://github.com/canonical-cloud/canonical-interfaces/pull/27), merge `3415d6d97721b18ee6734659d73a95ff3d35a151` | Merged. Versioned request, response, list/detail/retry, error, and WebSocket contracts with generated adapters. |
| Cross-language Dart/conformance | [`canonical-interfaces#29`](https://github.com/canonical-cloud/canonical-interfaces/pull/29), merge `140ce97ed6d0325a2308ed7fc4c54287ea755abb` | Merged. Complete Dart generation and Rust/Dart round-trip coverage. |
| Quote schema merge guards | [`canonical-interfaces#30`](https://github.com/canonical-cloud/canonical-interfaces/pull/30), merge `c60f3796e810b0448e84304afd009f1ac44f3ab6` | Merged after exact-head checks. Rejects null structural schema keywords, duplicate Dart generation, and order-sensitive definition inventories. |
| Cross-surface delivery guidance | [`canonical-web-server.rs#54`](https://github.com/canonical-cloud/canonical-web-server.rs/pull/54), merge `d936385af4242463a42f6fe34dfe104ae6a3f560` | Merged after exact-head documentation checks. Records web, mobile, desktop, and native evidence-agent boundaries without claiming unpublished clients. |

## Active deployment and certification lanes

| Work item | Purpose | Current gate |
| --- | --- | --- |
| [`k8s-cluster#1081`](https://github.com/ORESoftware/k8s-cluster/pull/1081) | Realm-isolated Shared Auth Argo CD application and rollout runbook. | Keep draft until database, secret, image, model-inventory, Cloudflare-inventory, TLS, and deployed-auth tests are proven. |
| [`k8s-cluster#1087`](https://github.com/ORESoftware/k8s-cluster/pull/1087) | Digest-pinned Canonical web/API origins and internal service wiring. | Keep draft and inactive. Republish the API from `f57528ccf2b077644917c1f770c97eca3027b8e7` and the web from `dcb979956a247f35a8470280717d0750109f2320`; record production and rollback digests before changing the pins. |
| [`k8s-cluster#1090`](https://github.com/ORESoftware/k8s-cluster/pull/1090) | Independent exact-head Shared Auth certification lane. | Blocked before private-source checkout because the documented read-only GitHub App Actions secrets are absent. Do not replace that boundary with a PAT. |
| [`canonical-monorepo#32`](https://github.com/canonical-cloud/canonical-monorepo/pull/32) | Pin the reviewed API/domain topology into the superproject. | Keep draft. Advance the API gitlink to `f57528ccf2b077644917c1f770c97eca3027b8e7` and the web gitlink to `dcb979956a247f35a8470280717d0750109f2320`, then rerun the recursive stack matrix. The missing `canonical-cloud-test` mirror still prevents independent staging certification. |

## Supersession and cleanup completed

- `canonical-interfaces#28` was closed as superseded. Its quote contract and Dart work landed through #27/#29, and the merge-fallout risks are protected by #30.
- `canonical-monorepo#31` was closed because it built the API from the obsolete web-workspace target rather than the standalone API repository.
- `canonical-monorepo#34` was closed because `canonical-infra` now exists and owns reviewed Cloudflare deployment source; inventory and activation remain tracked in `.github#13` and Linear.
- `k8s-cluster#1084` and `#1085` were closed as obsolete origin drafts. `#1087` is the corrected, digest-pinned successor and remains unactivated.
- `canonical-web-server.rs#55` was closed after docs-only PR #54 received exact-head green checks and merged through the protected path.
- The newer backlog Linear project named `github.com/canonical-cloud` was canceled as a duplicate. The older in-progress project linked above remains canonical.

The temporary `CANONICAL_WEB_SERVICE_TOKEN` compatibility alias remains intentionally staged for rollout safety. `DEN-2825` requires its removal after every Kubernetes, local, smoke, secret-store, canary, and rollback surface uses `CANONICAL_INTERNAL_AUTH_TOKEN` and the complete web matrix passes again.

## Activation gates

1. Apply the Canonical API and Shared Auth schemas with privileged one-shot migration identities; retain non-owner runtime identities afterward.
2. Seed exactly one active `canonical_context` row for `quote-analysis`.
3. Provision isolated Canonical Supabase, PostgreSQL, service-token, signing, sealing, Redis, webhook, introspection, and Gemini secrets through approved secret stores.
4. Publish and record immutable production and rollback digests for every deployed process from the reviewed source heads above.
5. Confirm the selected Google project and region expose the exact `gemini-3.6-pro` model identifier. Do not silently substitute another model; retain `GEMINI_MODEL` as an explicit operator override.
6. Authenticate to the authorized Cloudflare account, prove exact `canonical.plus` zone ownership, and inventory the exact Worker environment, routes, DNS records, and intended Kubernetes origin before any write.
7. Require healthy origin TLS before adding or changing proxied DNS or Worker routes.
8. Certify login, sealed return, CSRF rejection, refresh/revocation, owner isolation, quote submission, model failure behavior, REST recovery, and WebSocket updates in the deployed environment.
9. Keep R2 out of scope unless a reviewed quote-specific bucket and data-retention requirement are explicitly defined.

## Current blockers

- `canonical-cloud-test` currently returns 404, so the planned no-secrets exact-head staging mirror cannot run.
- The prior Canonical Cloudflare credential failed authentication; no account, zone, route, DNS, Worker, or R2 state was changed.
- The exact `gemini-3.6-pro` deployment-project inventory has not been proven.
- The independent Shared Auth certification lane lacks its narrowly scoped GitHub App Actions secrets.
- Persistent database migrations, production secrets, immutable image publication, origin health/TLS, Cloudflare inventory, and deployed end-to-end certification remain operator-gated.

No live Cloudflare, DNS, R2, Kubernetes, Supabase, persistent PostgreSQL, secret-store, or production-model mutation is represented as completed by this ledger.
