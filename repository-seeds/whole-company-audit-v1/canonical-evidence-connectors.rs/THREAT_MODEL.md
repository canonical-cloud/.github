# Threat model

## Assets

- customer tenant and organization boundaries;
- credential confidentiality and least-privilege upstream authority;
- bounded, normalized evidence with reliable provenance and freshness; and
- the guarantee that collection cannot mutate customer systems.

## Trust boundaries

Upstream API records, fixture JSON, pagination links, rate-limit responses, and environment configuration are untrusted. Credential values and production tenants are deliberately outside this seed. The auditor is a separate trust boundary and revalidates schema, tenant/scope, and content digests at import.

## Threats and controls

| Threat | Seed control | Production follow-up |
| --- | --- | --- |
| Raw credential enters arguments, evidence, or logs | Public configuration models only `env:` and `secret-store:` references; parse failures omit values | Resolve credentials in an isolated secret provider with structured log redaction |
| Accidental upstream mutation | Transport surface has only `get_page`; descriptor asserts read-only scopes | Enforce GET/HEAD at the HTTP client and test against a deny-by-default fake server |
| Ambient or wrong tenant collection | Tenant and organization are required and validated; there are no defaults | Bind configuration to authenticated job identity and customer authorization |
| Sensitive raw fields escape | Normalization uses an exact seven-field allow-list; property tests inject arbitrary unknown fields | Add data-classification review and retention policy per connector |
| Unbounded pagination or memory growth | One-based monotonic pages, ten-page/1,000-record bounds | Add byte limits and streaming backpressure at the service boundary |
| Silent rate-limit truncation | Typed retry-after error aborts the entire collection | Persist collection checkpoints without publishing partial evidence |
| Evidence tampering after collection | Canonical content SHA-256 accompanies every observation and is rechecked by the auditor | Add signed collector attestations and verified workload identity |
| Production access from CI | Only hermetic fixture transport is implemented | Keep production integration tests in an isolated authorized environment |
| Framework coupling | Connector emits framework-neutral evidence types only | Keep mappings in `canonical-audit-programs` |

## Non-goals and deferred controls

This seed does not implement live HTTP, secret resolution, OAuth installation flows, signatures, encryption, customer retention, checkpoint persistence, or the non-GitHub adapter families. Those require separately reviewed repository code, an authorized test organization, and production identity/secret infrastructure after `DEN-319`. The absent production transport is an intentional fail-closed control, not an implicit deployment gap that callers may bypass.
