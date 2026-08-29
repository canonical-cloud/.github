# canonical-company-auditor.rs

Framework-neutral Rust domain and orchestration primitives for whole-company assessments. This seed is tracked by `DEN-1721` and is not yet a published repository or deployed service.

## Boundary

This crate owns:

- explicit tenant and hierarchical scope types;
- least-privilege import, package-assembly, and export capabilities;
- subjects covering organizations, business units, systems, data classes, identities, vendors, policies, risks, controls, exceptions, findings, and remediation owners;
- assessment periods and uniform manual/automated evidence provenance;
- a versioned connector observation input, canonical SHA-256 verification, and deterministic observation IDs;
- deterministic immutable audit package manifests with create-only persistence; and
- JSON Schema and an OSCAL-compatible back-matter projection.

It does not own framework content, control mappings, connector transports, remediation, user authentication, or authorization policy. Framework overlays belong in `canonical-audit-programs`; connector implementations belong in `canonical-evidence-connectors.rs`.

## CLI harness

`auditorctl` reads JSON from standard input and writes JSON to standard output. It accepts no credential values and has no network access.

```text
auditorctl package <tenant> <scope> <starts-at> <ends-at>
auditorctl verify
auditorctl oscal
```

The `package` input is `canonical.evidence-observation-batch/v1`. `verify` and `oscal` accept a serialized `AuditPackage` and reject any mismatch before producing output.

## Security invariants

- Tenant and scope are required; there is no ambient or default tenant.
- Capability checks bind operation, tenant, and segment-aware scope.
- Normalized observation content is an object bounded to 64 KiB of canonical JSON.
- The source-supplied content digest is recomputed at import.
- Package construction sorts observations, rejects duplicates, recomputes digests, and derives manifest/package identities deterministically.
- Persistence uses create-new semantics and refuses overwrite.
- Deserialized packages are untrusted until `verify` succeeds.

See [THREAT_MODEL.md](THREAT_MODEL.md) for trust boundaries and deferred production controls.
