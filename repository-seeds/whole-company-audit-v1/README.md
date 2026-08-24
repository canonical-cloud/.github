# Whole-company audit repositories — production seed v1

This directory is a reproducible, reviewable source seed for two repositories that do not yet exist:

- `canonical-cloud/canonical-company-auditor.rs` (`DEN-1721`)
- `canonical-cloud/canonical-evidence-connectors.rs` (`DEN-1724`)

It is not a deployed service and does not claim that either target repository has been provisioned. Repository publication remains blocked by `DEN-319`; no credential or administration bypass is included here.

## Proven vertical slice

The two independent Rust crates exercise one framework-neutral flow:

1. a hermetic, read-only GitHub posture adapter accepts bounded fixture pages;
2. the connector applies allow-list redaction and emits normalized `EvidenceObservationInputV1` values;
3. the auditor imports that owned boundary, verifies tenant/scope/provenance/content digests, and deterministically builds an immutable audit package;
4. package persistence refuses overwrite and verifies the manifest after reload; and
5. concurrency and property tests prove order-independent package identity and tamper detection.

The connector deliberately exposes no mutating HTTP method. A live GitHub authority configuration can be loaded only from named environment values and only accepts an `env:` or `secret-store:` credential reference. This seed contains no production HTTP transport; live collection fails closed until a separately reviewed transport is wired in the published connector repository.

Framework catalogs and mappings are intentionally absent. They belong to `canonical-audit-programs`, not either domain model.

## Validate

Requirements: Rust `1.95.0`, Python 3, and `cargo-audit 0.22.2`.

```bash
bash ./validate.sh
```

The script runs formatting, strict Clippy, locked tests, dependency audits, and the cross-repository CLI conformance flow. It does not contact a tenant, resolve a secret, or write outside Cargo's normal build cache.

## Publication order after DEN-319

1. Create both empty public repositories through the authorized organization-administration workflow.
2. Copy each named source directory into its matching repository without its build output.
3. Commit and publish `canonical-company-auditor.rs` first; run its locked CI and record the immutable commit.
4. Re-run connector conformance against that exact auditor commit. The production integration boundary is the versioned JSON exchange schema, so no source-level path dependency needs rewriting.
5. Commit and publish `canonical-evidence-connectors.rs`; run its locked CI.
6. Record both repository heads and hosted workflow runs in `DEN-1721` and `DEN-1724`.
7. Keep the issues blocked or in progress until the actual repositories exist; this carrier PR alone is not production publication.

No force push, rebase, history replacement, default-branch write, credential persistence, or target-repository creation is performed by this seed.
