# Desktop application allocation

Verified **2026-08-05**.

Canonical Cloud **might** benefit from paired native local evidence-collection applications. The primary audit-management experience remains web/CLI-first:

- Rust: [`canonical-cloud/canonical-agent-desktop.rs`](https://github.com/canonical-cloud/canonical-agent-desktop.rs) — **proposed**, not yet verified as a published repository.
- Flutter: [`canonical-cloud/canonical-agent-flutter`](https://github.com/canonical-cloud/canonical-agent-flutter) — **proposed**, not yet verified as a published repository.

These names are optional allocation targets, not proof that either remote exists and not a commitment to build a full desktop management UI. Native clients should be promoted only when a signed local evidence collector materially improves filesystem inventories, local policy checks, browser/device attestations, watch folders, secure uploads, operator prompts, or offline collection.

## Potential product boundary

A future pair could cover semantic parity for evidence-source selection, local inventory, policy and configuration checks, attestations, redaction, evidence manifests, checksums, secure upload queues, consent and operator prompts, offline collection, retry/recovery, and audit receipts.

A shared Rust collection and policy engine may sit behind an explicit library, FFI, or local-service boundary, but any Flutter application must remain independently buildable, testable, and releasable. Shared schemas, evidence manifests, policy-result formats, fixtures, sample evidence, golden receipts, and conformance tests should be versioned deliberately.

## Promotion rule

Promote this pair from optional proposal to planned only when the local-agent workflow, signing and update model, privacy/security boundary, ownership, milestones, and repository creation are accepted. Once planned, desktop-facing changes must inspect both implementations, define shared acceptance and security criteria, update both or record an explicit no-change rationale, and report Rust and Flutter status separately.

## Project routing

- GitHub Project: [`canonical-cloud-project` — Project 1](https://github.com/orgs/canonical-cloud/projects/1)
- Linear project: `github.com/canonical-cloud`
- Central registry: [`ORESoftware/project-registry`](https://github.com/ORESoftware/project-registry/blob/main/registry/desktop-applications.json)
- Portfolio rollout: [`DEN-2469`](https://linear.app/denman/issue/DEN-2469/roll-out-paired-rust-flutter-desktop-repositories-across-the-portfolio)

Promotion, repository creation, renames, transfers, archival, signing/update changes, or platform-status changes must update this document, Linear, the central registry, and both companion repositories together.
