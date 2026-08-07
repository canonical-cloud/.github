# Canonical Plus authenticated quote execution ledger — August 7, 2026

This ledger reconciles the active Canonical Plus quote work across GitHub, Linear, the Canonical organization Project, Shared Auth, and `ORESoftware/k8s-cluster`.

## Planning authorities

- GitHub organization: [`canonical-cloud`](https://github.com/canonical-cloud)
- GitHub Project: [`canonical-cloud-project`](https://github.com/orgs/canonical-cloud/projects/1)
- Canonical Linear project: [`github.com/canonical-cloud`](https://linear.app/denman/project/githubcomcanonical-cloud-1659c8ea1adf)
- Stable Project routing card: [`.github#2`](https://github.com/canonical-cloud/.github/issues/2)
- Primary Linear delivery issue: `DEN-2622`
- Quote-contract convergence issue: `DEN-2642`
- Kubernetes activation issue: `DEN-2601`
- Cloudflare inventory and routing issues: `DEN-2631`, `DEN-2649`, and `DEN-2655`

GitHub is authoritative for repositories, commits, pull requests, checks, artifacts, and deployment evidence. Linear is authoritative for scope, priority, ownership, dependencies, milestones, and status. The organization Project is the cross-repository execution view.

## Landed source boundaries

| Boundary | Evidence | State |
| --- | --- | --- |
| Shared Auth browser ceremony | [`shared-auth-server.rs#41`](https://github.com/shared-auth/shared-auth-server.rs/pull/41), merge `22aab1de937620251f4e0b9a617c485733c97ff5` | Merged. Realm-specific magic-link/OTP login, sealed relative returns, host-only cookies, refresh, and Canonical overlay. |
| Shared Auth edge behavior | [`shared-auth-infra#9`](https://github.com/shared-auth/shared-auth-infra/pull/9), merge `6234f1ee72349f84652c85a5a957b2982ea471bf` | Merged. Canonical issuer/audience/cookie namespace, refresh verification, header stripping, and 12/12 Node Worker tests. |
| Signed-in web application | [`canonical-web-server.rs#41`](https://github.com/canonical-cloud/canonical-web-server.rs/pull/41), merge `74448c8dcb885fbb240ac59d1079a929bd06caa5` | Merged. Maud/HTMX `/u/quote`, origin authorization, CSRF, forced RLS, REST recovery, WebSocket hints, and configurable `gemini-3.6-pro` default. |
| Durable quote API | [`canonical-api-server.rs#6`](https://github.com/canonical-cloud/canonical-api-server.rs/pull/6), merge `91ac093bf6c3d0958918fc8678af95dd13975f1e` | Merged. Durable PostgreSQL queue, leases/recovery, owner-scoped REST/WebSocket API, bounded Gemini analysis, and non-root container. |
| Shared context key | [`canonical-api-server.rs#8`](https://github.com/canonical-cloud/canonical-api-server.rs/pull/8), merge `e3a7cc79b3ceac0e455b9d7822a29d4154c9584b` | Merged. Web and API default to one active `quote-analysis` context record. |
| Canonical edge deployment source | [`canonical-infra#4`](https://github.com/canonical-cloud/canonical-infra/pull/4), merge `03d37469a6ea5ee075a89c064ee60017ae4ebf23` | Merged. Byte-verified reviewed Worker source, route/realm/cookie contracts, and provenance lock. No live Cloudflare write. |
| Quote-v1 wire authority | [`canonical-interfaces#27`](https://github.com/canonical-cloud/canonical-interfaces/pull/27), merge `3415d6d97721b18ee6734659d73a95ff3d35a151` | Merged. Versioned request, response, list/detail/retry, error, and WebSocket contracts with generated adapters. |
| Cross-language Dart/conformance | [`canonical-interfaces#29`](https://github.com/canonical-cloud/canonical-interfaces/pull/29), merge `140ce97ed6d0325a2308ed7fc4c54287ea755abb` | Merged. Complete Dart generation and Rust/Dart round-trip coverage. |

## Active implementation and certification lanes

| Work item | Purpose | Current gate |
| --- | --- | --- |
| [`canonical-interfaces#30`](https://github.com/canonical-cloud/canonical-interfaces/pull/30) | Regression guards against null JSON Schema keywords, duplicate Dart generation, and order-sensitive quote type checks. | Run the normal interface test and generated-artifact matrix; merge after exact-head checks pass. |
| [`canonical-web-server.rs#49`](https://github.com/canonical-cloud/canonical-web-server.rs/pull/49) | Delegate the signed-in quote UI to the standalone durable API while retaining origin Shared Auth and CSRF enforcement. | Must consume the released quote-v1 contract and pass exact web/API compatibility tests before merge. |
| [`canonical-web-server.rs#54`](https://github.com/canonical-cloud/canonical-web-server.rs/pull/54) | Cross-surface mobile/desktop delivery guidance. | Source CI and hierarchy checks pass, but stale required contexts block a docs-only merge; tracked by [`#55`](https://github.com/canonical-cloud/canonical-web-server.rs/issues/55). |
| [`k8s-cluster#1081`](https://github.com/ORESoftware/k8s-cluster/pull/1081) | Realm-isolated Shared Auth Argo CD application and rollout runbook. | Keep draft until database, secret, image, model-inventory, Cloudflare-inventory, TLS, and deployed-auth tests are proven. |
| [`k8s-cluster#1087`](https://github.com/ORESoftware/k8s-cluster/pull/1087) | Digest-pinned Canonical web/API origins and internal service wiring. | Keep draft and inactive until exact contract convergence and all activation gates are satisfied. |
| [`k8s-cluster#1090`](https://github.com/ORESoftware/k8s-cluster/pull/1090) | Independent exact-head Shared Auth certification lane. | Blocked before private-source checkout because the documented read-only GitHub App Actions secrets are absent. Do not replace that boundary with a PAT. |
| [`canonical-monorepo#32`](https://github.com/canonical-cloud/canonical-monorepo/pull/32) | Pin the reviewed API/domain topology into the superproject. | Keep draft until leaf contracts converge and the missing `canonical-cloud-test` mirror can certify exact heads. |

## Supersession and cleanup

- `canonical-interfaces#28` is based before merged PR #29 and now differs from `main` only by local-ignore material. Reconcile any still-useful ignore entries separately, then close the stale multi-commit carrier.
- `canonical-monorepo#31` builds the API from the obsolete web-workspace target. Port any release-ledger and attestation intent to the standalone API gitlink, then close it.
- `canonical-monorepo#34` is a temporary Cloudflare preflight carrier whose body says `canonical-infra` does not exist. Move reusable GET-only inventory logic to `canonical-infra`, then close it.
- `k8s-cluster#1084` and `#1085` are older Canonical origin drafts. `#1087` is the corrected, digest-pinned successor; the older duplicates should remain unmerged and be closed as superseded.
- The newer backlog Linear project named `github.com/canonical-cloud` is a duplicate. The in-progress project linked above remains canonical.

## Activation gates

1. Apply the Canonical API and Shared Auth schemas with privileged one-shot migration identities; retain non-owner runtime identities afterward.
2. Seed exactly one active `canonical_context` row for `quote-analysis`.
3. Provision isolated Canonical Supabase, PostgreSQL, service-token, signing, sealing, Redis, webhook, introspection, and Gemini secrets through approved secret stores.
4. Record immutable production and rollback digests for every deployed process.
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
- Persistent database migrations, production secrets, origin health/TLS, Cloudflare inventory, and deployed end-to-end certification remain operator-gated.

No live Cloudflare, DNS, R2, Kubernetes, Supabase, persistent PostgreSQL, secret-store, or production-model mutation is represented as completed by this ledger.
