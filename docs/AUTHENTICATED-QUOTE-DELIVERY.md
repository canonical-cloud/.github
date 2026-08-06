# Authenticated compliance quote delivery map — August 5, 2026

## Canonical planning and delivery links

- [GitHub organization](https://github.com/canonical-cloud)
- [GitHub Project: canonical-cloud-project](https://github.com/orgs/canonical-cloud/projects/1)
- [Linear project: github.com/canonical-cloud](https://linear.app/denman/project/githubcomcanonical-cloud-1659c8ea1adf)
- [Organization documentation](https://github.com/canonical-cloud/.github)

GitHub is authoritative for repositories, reviewed commits, pull requests, checks, merge state, artifacts, and runtime evidence. Linear is authoritative for planning, ownership, dependencies, milestones, blockers, and status. The organization Project is the cross-repository execution view.

## Delivered vertical slice

| Boundary | Repository evidence | Status |
| --- | --- | --- |
| Public quote CTA | [`canonical-marketing-site.web#21`](https://github.com/canonical-cloud/canonical-marketing-site.web/pull/21), merge `06e8dbbe64e6c8974a75acdae4573c2115b351d9` | Merged; every CTA routes to `https://app.canonical.plus/u/quote` without receiving credentials |
| Quote contracts | [`canonical-interfaces#24`](https://github.com/canonical-cloud/canonical-interfaces/pull/24), merge `ec2c739092c955e4756d2d692ef225adf67647e4` | Merged; bounded v1 request, progress, estimate, and public-problem contracts plus generated adapters |
| Web/API platform | [`canonical-web-server.rs#41`](https://github.com/canonical-cloud/canonical-web-server.rs/pull/41), merge `74448c8dcb885fbb240ac59d1079a929bd06caa5` | Merged; `/u/quote`, Shared Auth, owner-scoped PostgreSQL/RLS, Gemini analysis, REST, and authenticated WebSocket status |
| Edge and Kubernetes infrastructure | [`canonical-infra#2`](https://github.com/canonical-cloud/canonical-infra/pull/2), merge `286353c83822cc3d31684303f6d9155f4cbbcdf6` | Merged; Worker source/tests and Kubernetes/Argo CD definitions are review-ready but deliberately not deployed |
| Cross-repository contracts | [`canonical-e2e#2`](https://github.com/canonical-cloud/canonical-e2e/pull/2), merge `811c703e8602fa56eb7e6ee36937bf1eb001311c` | Merged; nine quote, edge, and exact Cloudflare-scope journeys pass |

## Newly provisioned private repositories

The following exact gaps were created in `canonical-cloud`; no repository outside the authorized organization was touched.

| Repository | GitHub repository ID | Purpose |
| --- | ---: | --- |
| [`canonical-flutter`](https://github.com/canonical-cloud/canonical-flutter) | `1324702771` | Paired Flutter mobile/desktop quote client |
| [`canonical-infra`](https://github.com/canonical-cloud/canonical-infra) | `1324702792` | Canonical Cloudflare and Kubernetes definitions |
| [`canonical-e2e`](https://github.com/canonical-cloud/canonical-e2e) | `1324702818` | Deterministic cross-repository product and deployment-scope contracts |

All three repositories are private and default to `main`.

## Changes under review

| Pull request | Exact purpose | Merge gate |
| --- | --- | --- |
| [`canonical-flutter#1`](https://github.com/canonical-cloud/canonical-flutter/pull/1) | Typed Flutter quote models, bounded API client, scoped token provider, mobile/desktop UI, Zed metadata, and focused tests | Exact Flutter tag/commit CI must pass before merge |
| [`canonical-clients#20`](https://github.com/canonical-cloud/canonical-clients/pull/20) | Rust and TypeScript quote transports integrated into the existing polyglot SDK matrix | Existing Rust, TypeScript, security, language-matrix, and Zed checks must pass |

## Existing staged follow-ups

These items already exist and should be advanced rather than duplicated:

- `canonical-api-server.rs#2` — draft standalone API extraction.
- `canonical-monorepo#32` — draft staging-mirror/submodule integration.
- `canonical-cli#10` — draft quote-contract validation through `canonical-lib`.

Their current staging design depends on `canonical-cloud-test`. That organization is outside the authorization scope of this task, so it was neither created nor modified here.

## Security and authority boundaries

- Cloudflare is routing and defense in depth; the origin and API independently verify authorization.
- Quote owner and tenant identity come from verified Shared Auth credentials, never from client-supplied payload fields.
- PostgreSQL with forced RLS is authoritative. WebSocket messages are wake hints; clients recover through REST.
- Gemini receives bounded untrusted input and produces a preliminary estimate for human review, not an audit opinion, certification, attestation, or legal conclusion.
- SDKs receive short-lived delegated tokens only. Provider refresh tokens, model keys, database credentials, and service credentials remain in platform secret stores.
- The Worker strips forged Canonical identity headers and treats an opaque session cookie only as an origin-validation hint.

## Cloudflare and R2 deployment status

A Canonical-only private inventory run authenticated successfully to GitHub and created the three repositories above. The supplied Cloudflare API token returned HTTP 401 before account/zone inventory completed.

Consequently, none of the following has been claimed or changed:

- account `62b833940607839add74bd2379cac303` inventory;
- exact `canonical.plus` zone ownership;
- Worker `canonical-app-router`;
- route `app.canonical.plus/*`;
- `app`, `api`, `auth`, or `origin-app` DNS records;
- Worker environments or secrets;
- R2 bucket inventory or objects.

No Cloudflare or R2 write occurred. R2 credentials were not used because no exact bucket name was proven. The blocker is tracked in [`canonical-infra#3`](https://github.com/canonical-cloud/canonical-infra/issues/3) and Linear issue `DEN-2631`.

## Production activation gates

1. Replace or repair the Canonical-scoped Cloudflare API token.
2. Verify the active token, exact account ID, and one exact `canonical.plus` zone owned by that account.
3. Inventory the exact Worker script, route, DNS records, environments, and R2 buckets without mutation.
4. Replace every production image placeholder with a reviewed immutable digest.
5. Verify Shared Auth realm/audience, introspection secret, PostgreSQL migration role, runtime role, and ExternalSecret references.
6. Confirm the selected Gemini model is enabled in the deployment project and region.
7. Run protected live browser/API/Kubernetes tests and record non-secret evidence in GitHub and Linear.
8. Review any Cloudflare or R2 write separately; never infer a target from naming alone.
