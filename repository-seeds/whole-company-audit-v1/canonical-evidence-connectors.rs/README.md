# canonical-evidence-connectors.rs

Least-privilege, read-only enterprise evidence connector primitives. This seed is tracked by `DEN-1724` and is not yet a published repository or deployed service.

## Implemented vertical slice

The GitHub adapter collects repository posture through a GET-only transport trait and emits the versioned JSON observation input owned by `canonical-company-auditor.rs`. It covers repository identity, visibility, default branch, required branch protection, Actions availability, team count, and an opaque audit-log reference.

The adapter:

- declares exact permission scopes, evidence types, freshness, pagination, record limits, rate-limit behavior, and redaction allow-list;
- applies a seven-field allow-list and drops all unrecognized upstream content;
- produces canonical SHA-256 content digests before handoff;
- sorts normalized observations deterministically;
- fails closed on malformed pages, record overflow, rate limiting, and freshness overflow; and
- is tested only with a hermetic fixture transport.

The generic `ConnectorFamily` boundary reserves independently reviewable adapters for AWS, GCP, Azure, identity providers, Google Workspace, Microsoft 365, Kubernetes, PostgreSQL/managed databases, endpoints, vulnerabilities, ticketing, vendor management, and policy documents. Those adapters are not falsely claimed as implemented by this seed.

## Credential and live-authority boundary

Credential material has no type or argument in this API. `CredentialReference` accepts only:

- `env:VARIABLE_NAME`; or
- `secret-store:provider/path`.

The CLI never accepts either a reference or a credential value for fixture collection. `LiveGithubAuthority::from_environment` requires explicit tenant, organization, and credential reference names. This seed intentionally includes no production HTTP transport, so `live-check` fails closed even when configuration exists. A live transport requires a separate security review after `DEN-319` repository publication.

## CLI harness

```text
connectorctl collect-fixture <path> <tenant> <organization> <collected-at>
connectorctl live-check
```

Fixture collection writes a `canonical.evidence-observation-batch/v1` document to standard output. It performs no network request and resolves no secret.

See [THREAT_MODEL.md](THREAT_MODEL.md) for trust boundaries and deferred production controls.
