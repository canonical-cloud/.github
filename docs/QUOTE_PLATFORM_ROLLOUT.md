# Canonical quote platform rollout

This page links the authoritative planning, implementation, and rollout records
for the signed-in quote flow at `app.canonical.plus/u/quote`. It does not contain
secrets, credential identifiers, platform object IDs, or mutable deployment
values.

## Authoritative records

| Concern | Record |
| --- | --- |
| Product program | [Linear project: github.com/canonical-cloud](https://linear.app/denman/project/githubcomcanonical-cloud-93dba9812ffe) |
| Architecture and acceptance contract | [Linear document: Canonical quote platform architecture and rollout](https://linear.app/denman/document/canonical-quote-platform-architecture-and-rollout-a87a8fe9131a) |
| Edge, infrastructure, and Kubernetes rollout | [Linear DEN-2649](https://linear.app/denman/issue/DEN-2649/create-canonical-infra-and-deploy-guarded-canonicalplus-edge-routing) and [GitHub issue #13](https://github.com/canonical-cloud/.github/issues/13) |
| Durable abandoned-job recovery | [Linear DEN-2650](https://linear.app/denman/issue/DEN-2650/add-leased-recovery-worker-for-abandoned-quote-analyses) |
| Durable quote API foundation | [`canonical-api-server.rs` PR #10](https://github.com/canonical-cloud/canonical-api-server.rs/pull/10) |
| Browser-safe quote API contract | [`canonical-api-server.rs` PR #11](https://github.com/canonical-cloud/canonical-api-server.rs/pull/11) |

The organization GitHub Project remains the portfolio view. Its exact URL and
node identifier must be discovered from the authenticated organization context
and stored in organization automation variables; do not guess a project number
or commit a mutable platform identifier.

## System boundaries

### Marketing

`canonical-marketing-site.web` owns the public CTA. The destination is
`https://app.canonical.plus/u/quote` and the copy promises a quote in under five
minutes.

### Shared Auth

The existing Canonical Shared Auth Kubernetes overlay owns email magic links,
email OTP, remembered verified-email choices, SMS OTP, authenticator-app TOTP,
WebAuthn passkeys, 3FA-compatible `otpauth` import, AAL/AMR/ACR, issuer/audience
validation, and session revocation.

Canonical uses the same-origin `/shared-auth` prefix. Anonymous `/u/*` requests
return through a relative path such as:

```text
/shared-auth/auth/browser/sign-in?return=/u/quote
```

### Web tier

`canonical-web-server.rs` owns Maud/Axum/HTMX pages and authoritative origin
session verification. It may project only the verified Shared Auth subject to
the private API boundary. The browser cannot choose the subject, internal
service token, Canonical context row, application Markdown, Gemini key/model,
or database credentials.

### API tier

`canonical-api-server.rs` owns quote REST/WebSocket endpoints, owner-scoped
PostgreSQL persistence, forced RLS, immutable context snapshots, structured
Gemini analysis, append-only status events, and model-attempt metadata.

The web-to-API contract uses:

```text
x-canonical-internal-token
x-canonical-subject
```

Both headers are server-generated. Edge and ingress configuration must remove
client-supplied `x-auth-*` and `x-canonical-*` headers.

## Repository topology

Deployable repositories belong as pinned git submodules under
`canonical-monorepo/apps/`:

- `canonical-web-server.rs`;
- `canonical-api-server.rs`;
- `canonical-marketing-site.web`;
- `canonical-mcp-server.rs`;
- future `canonical-infra`, after that repository exists.

Reusable code remains a Zed dependency graph rather than duplicate submodules:

```text
canonical-interfaces -> canonical-lib -> API/web/CLI/MCP
canonical-interfaces -> canonical-clients -> canonical-cli
```

`canonical.cloud` is a superseded compatibility mirror and must not receive new
application architecture.

## Deployment gate

No Cloudflare write is permitted until an operator proves, from returned
platform data, the exact authorized Canonical account, exact `canonical.plus`
zone ownership, Worker project, environment, route, DNS record, and any R2
bucket. Ambiguity is a stop condition. No non-Canonical account, domain, zone,
project, Worker, route, record, or bucket may be changed.

Kubernetes rollout must pin immutable image digests and inject every credential
through the platform secret store. Required staging evidence includes:

- magic-link and email-OTP return to `/u/quote`;
- encrypted remembered-email dropdown behavior;
- passkey, TOTP, SMS, and 3FA import enrollment;
- revocation, expiry, wrong-audience, and insufficient-AAL failures;
- owner isolation and spoofed-header rejection;
- every supported compliance framework;
- Gemini/provider failure and bounded error behavior;
- REST recovery and WebSocket reconnect;
- deployment rollback.

## Known follow-up

Quote records and events are durable, but an in-flight Gemini call currently
runs in-process. DEN-2650 tracks a PostgreSQL lease/heartbeat/retry worker so
abandoned `queued` or `analyzing` work becomes safely reclaimable after pod
failure.
