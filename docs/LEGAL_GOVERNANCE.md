# Legal governance and executable contract policy

`canonical-cloud/canonical-docs` is the reviewable legal workspace for Canonical Cloud. `ores-legal` is a reusable drafting/governance pattern source, not a source of automatically binding terms. The governed legal catalog remains the broad intake/routing inventory; the executable contract layer described here binds selected legal artifacts to software and release surfaces.

## Contract authorities

The standard machine-readable layer consists of independent `contracts/main.tsp` and `contracts/authored.schema.json` authorities, a `LegalRegistry` corpus, and deterministic human/runtime projections in `canonical-docs`. `tjsv` compares the authorities and validator behavior; generated schema is evidence only. `oresc` performs repository/governance integration and delegates contract parity to `tjsv`.

Only `approved` end-user-distributed documents can enter the runtime approved manifest. Consumers bind to document ID plus exact content hash and fail closed on missing, blocked, stale, or mismatched acceptance metadata. `draft`, `review_required`, and `gap` never become runtime-approved just to satisfy CI.

## Evidence and assurance boundary

A cloud observation, scanner output, configuration snapshot, webhook payload, control mapping, report, or customer assertion is evidence about a source at a point in time. It is not by itself a passed control, legal-compliance conclusion, certification, attestation, audit opinion, or guarantee of future state.

Changes to collectors, agents/sidecars, CLIs, connectors, report generation, IAM scopes, credentials, network access, telemetry, subprocessors, retention, data residency, customer-hosted execution, remediation actions, or evidence claims require legal-impact review. Read-only inspection must remain distinguishable from mutating cloud operations.

## SLA, SLO, and SOW

SLOs are engineering objectives. SLA availability/support commitments, credits, exclusions, measurement rules, maintenance windows, and remedies are contractual. SOWs carry engagement scope, deliverables, assumptions, customer dependencies, acceptance, fees, and change control. Code, dashboards, or config constants do not silently create or supersede those terms.

## Workforce and repository privacy boundary

Employment and contractor templates, onboarding/offboarding, separation, and termination materials must be jurisdiction-routed. Never commit executed agreements, signatures, customer evidence, credentials, privileged communications, personnel records, identity documents, or matter-specific legal advice. Git may store blank templates, policy text, catalog/routing metadata, approval references, and deterministic hashes.

## Reusable CI

Repositories using the standard legal contract layout can call:

```yaml
jobs:
  legal-contract:
    uses: canonical-cloud/.github/.github/workflows/reusable-legal-contract-audit.yml@main
```

Private/release runners should set `require_ores_cli: true` after provisioning `oresc`. The reusable check verifies synchronization and runtime admission metadata; substantive legal sufficiency remains subject to qualified counsel and accountable business approval.
