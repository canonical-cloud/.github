# Threat model

## Assets

- tenant isolation and the authorized assessment scope;
- evidence provenance, freshness, normalized content, and digests;
- immutable package identity and persisted manifests; and
- the separation between framework-neutral facts and framework mappings.

## Trust boundaries

Connector and manual evidence JSON is untrusted. Deserialized packages are untrusted. Capability grants are trusted only when created by a future authenticated service boundary; this pure crate does not authenticate actors. Filesystem durability and access control remain responsibilities of the hosting service.

## Threats and controls

| Threat | Seed control | Production follow-up |
| --- | --- | --- |
| Cross-tenant evidence injection | Every import requires a matching tenant, segment-aware scope, and `ImportEvidence` grant | Bind grants to verified service identity claims |
| Prefix-confused scope escalation | Scope containment checks whole slash-delimited segments | Add policy conformance against the production authorization service |
| Tampered evidence | Canonical JSON SHA-256 is verified at import and package verification | Sign collector attestations and retain key identity/history |
| Stale evidence represented as current | Collection and validity timestamps are required and ordered | Enforce program-specific freshness at assessment evaluation time |
| Memory or package amplification | Normalized content is capped at 64 KiB per observation | Add package count/total-byte quotas at the service boundary |
| Manifest nondeterminism | Observations are sorted by deterministic ID before manifest hashing | Maintain cross-language golden fixtures |
| Package overwrite or rollback | Persistence uses create-new semantics and verifies after load | Store in append-only object storage with retention and monotonic ledgers |
| Framework text contaminates the domain | No framework identifiers or mappings exist in this crate | Review overlays only in `canonical-audit-programs` |
| External format becomes the internal model | JSON/OSCAL values are explicit boundary projections | Version and migrate adapters independently from domain types |

## Non-goals and deferred controls

This seed does not provide user authentication, key management, signatures, encryption at rest, database row-level security, distributed locking, retention enforcement, or an HTTP service. Those controls require the eventual repository, deployment identity, secret store, persistence layer, and production threat review. Publication and service wiring remain blocked by `DEN-319` and must not be inferred from this source seed.
