# Canonical Cloud legal document governance

> **PUBLIC-SAFE GOVERNANCE — NOT LEGAL ADVICE — NOT AN EXECUTED AGREEMENT**

Canonical Cloud keeps detailed legal drafting in the private `canonical-cloud/canonical-docs`
repository. This public organization repository contains policy and routing only. Do not
copy executed contracts, customer confidential information, employee records, privileged
legal advice, signatures, identity documents, financial data, secrets, or regulated
customer evidence here.

Linear: `DEN-1049` (legal documents), `DEN-628` (portfolio licensing). The shorter
organization standard is [`LEGAL_GOVERNANCE.md`](../LEGAL_GOVERNANCE.md); the
repository-maintainer routing guide is [`docs/LEGAL_AND_CONTRACTS.md`](../docs/LEGAL_AND_CONTRACTS.md).

## 1. Source boundaries and ownership

- `canonical-cloud/canonical-docs` owns Canonical Cloud legal templates, clause playbooks,
  legal-document metadata contracts, and legal audit tooling.
- `ores-legal/*` supplies reusable signing/evidence architecture and patterns; Canonical
  documents remain Canonical-specific and require their own counsel approval.
- Product/code repositories may reference stable legal document IDs and approved versions
  but should not fork legal prose into application source.
- Executed agreements belong in the governed records system, not Git.

| Concern | Authoritative home | Notes |
| --- | --- | --- |
| Legal templates (customer, partner, workforce, corporate) | `canonical-docs` → `docs/legal/{external,internal}` | Markdown is the drafting source. Every template carries a metadata header with a `Document ID`, `Template version`, `Status`, `Owner`, `Review cadence`, and `Signing` row. |
| 170-type agreement catalog, playbooks, review gates | `canonical-docs` → `docs/legal/catalog/` | Catalog presence routes a matter to a playbook; it never asserts an execution-ready form. |
| Service tiers, prices, support targets, SOW packages | `canonical-docs` → `data/legal/service-tiers.json` | The only place a price changes. The pricing schedule, SLA and support tables, SOW package menu, and the public `/prices/` page are all rendered from it. |
| Legal data contracts (TypeSpec + JSON Schema peer authorities) | `canonical-docs` → `contracts/` | Compared by `ORESoftware/typespec-json-schema-validator` (`tjsv`); audited by `ORESoftware/ores-cli` (`oresc`). Neither authority is generated from the other. |
| Legal document registry (data extracted from the templates) | `canonical-docs` → `generated/legal-document-registry.json` | Consumed by this repository, the ores-legal signing platform, and `ores-cli`. |
| Public prices page | `canonical-cloud.github.io` → `/prices/` | Renders the synced copy `src/data/service-tiers.json`; gated by email + one-time code. Never edited by hand — refreshed from `canonical-docs`. |
| Executed instruments, signatures, acceptance records, digests | The governed records system and the ores-legal signing platform | **Never this organization's repositories.** No executed copy, signature page, filled placeholder, cap table, customer name in a live SOW, or personal identifier is committed anywhere. |
| Licensing of repository content | `canonical-docs` → `LICENSE-POLICY.md` | Intentional no-license-grant posture pending `DEN-628`. |

## 2. Matter coverage

The legal program routes at least these matter classes:

- customer MSA/SOW/order forms, SaaS/cloud/service terms, SLA/support, amendments,
  renewals, termination and transition;
- privacy policy, DPA, security/privacy addenda, subprocessors, data sharing/licensing,
  retention/deletion and customer evidence terms;
- EULA/software/evaluation/API/open-source and other IP licensing;
- mutual and unilateral NDAs, confidentiality, invention assignment and IP ownership;
- customer contract exceptions, waivers, credits and negotiated deviations, and the
  liability waiver and release used for previews, on-site sessions and exercises;
- audit-tooling authorization (read-only access, output-is-not-assurance boundary) and the
  continuous-readiness evidence addendum;
- employment/contractor/advisor/founder agreements, offers, handbooks and policy notices;
- onboarding, offboarding, termination notices, severance/separation/releases, benefits,
  equity and workforce records;
- corporate governance, board/stockholder actions, equity administration and intercompany
  arrangements;
- vendor/partner/reseller/procurement and independent evaluator relationships;
- security, privacy, records, legal-hold, audit and compliance-readiness governance; and
- litigation/dispute, settlement, release, regulatory, finance, real-estate and specialist
  matters routed to qualified counsel rather than treated as generic self-service forms.

Catalog coverage means the matter is recognized and routed. It does not mean every matter
has an execution-ready form or that one template is enforceable in every jurisdiction.
`canonical-docs/scripts/check_legal.py` fails when a required maintained template is missing.

## 3. Signing routes

Every template declares exactly one route in its `Signing` header row. The route decides
what the executed record is; the Markdown is never it.

| Route | Meaning | Record |
| --- | --- | --- |
| `ores-legal-envelope` | Executed by the parties through an ores-legal envelope (MSA, SOW, SLA, DPA, NDAs, waivers, employment and separation documents, board consents). | Envelope identifier, completed-document digest, audit trail. |
| `counter-signed-order` | Incorporated by reference into a counter-signed Order Form (support addendum). | Order Form envelope and the referenced template version. |
| `acknowledgement` | Accepted by click-through or written acknowledgement (EULA, AUP, handbook, policies, checklists, audit-tooling terms). | Acceptance event with template version and time. |
| `not-signed` | Published, adopted, or filed elsewhere (ToS, privacy policy, SLOs, pricing schedule, charter documents, plans). | Adoption consent, filing receipt, or publication record. |

The ores-legal platform reads the registry's `signing` field to decide whether a template
is offered as an envelope template, an acknowledgement, or not at all. A template whose
header is incomplete never reaches the platform because the registry extractor fails
closed on it.

## 4. Review and approval

Legal language cannot be approved by AI, CI, a linter, a schema validator, a generated
receipt, a project-management state, or a source-control merge. Qualified counsel must
approve the exact matter-specific version when legal review is required. Security,
privacy, finance/tax, HR/employment, product, and business owners review their respective
facts and controls.

Approval evidence should bind the exact source revision/content digest, document ID and
version, reviewer role, jurisdiction/matter scope, decision reference, and decision time.
A later substantive edit invalidates stale approval.

Template versions follow semver: patch for wording, minor for a clarification or new
optional clause, major for changed risk allocation or legal effect. The agreement catalog
version and the pricing catalog version follow the same rule. Executed versions are never
overwritten; amendments, renewals, assignments, terminations, and corrections are new
linked records in the records system.

## 5. Docs ↔ code synchronization

For selected machine-readable legal metadata — not clause wording — Canonical Cloud uses:

- independently authored TypeSpec;
- independently authored JSON Schema Draft 2020-12;
- pinned `ORESoftware/typespec-json-schema-validator` parity and Contract IR evidence;
- deterministic Rust/TypeScript/reference-document projections; and
- `ORESoftware/ores-cli` repository/config/source governance when the private tool is
  provisioned in the CI trust boundary.

Docs from data: `data/legal/service-tiers.json` renders into the pricing schedule, the SLA
and support tables, the SOW package menu, and the marketing site's `/prices/` copy. Data
from docs: every template header is extracted into the legal document registry. Generated
output is derivative and must never overwrite either authored machine authority or
counsel-owned legal prose. A parity receipt proves only the configured machine contract
converged for the exact checked source.

Required checks before a legal change merges:

1. `python3 scripts/check_legal.py` — leaks, claims, required clauses, header hygiene,
   index integrity.
2. `python3 scripts/check_legal_catalog.py` — catalog structure and the readiness
   boundary phrases.
3. `python3 scripts/check_legal_data.py`, `render_legal_data.py --check`,
   `extract_legal_registry.py --check` — data satisfies the authored contract, documents
   match the data, registry matches the documents.
4. The `tjsv` peer-authority gate (pinned action) over `contracts/` with the instance
   corpus, emitting a digest-bound Contract IR.
5. `oresc audit repo --profile docs` and `oresc audit contract`, with any accepted finding
   recorded in the repository's accepted-findings file naming its governing decision.
6. For the site: `npm test` and `npm run build` in `canonical-cloud.github.io`, which fail if
   any page other than `/prices/` references Supabase or the gate, or if the built page
   disagrees with the synced catalog.

The selectable [`legal-contract-integrity`](../workflow-templates/legal-contract-integrity.yml)
workflow template and the reusable
[`reusable-legal-contract-audit.yml`](../.github/workflows/reusable-legal-contract-audit.yml)
workflow install checks 4 and 5 in any repository that carries legal/commercial contracts;
callers provide the least-privilege `ORES_TOOLING_READ_TOKEN` secret.

## 6. Waivers and exceptions

Internal scanner/audit waivers and customer contract waivers are different instruments.
An internal `ores-cli`, TJSV, security, or CI exception:

- must bind the exact finding/rule, immutable source revision, scope, risk, approver,
  compensating controls and expiry;
- must preserve the underlying finding and lineage;
- must not turn `failed`, `stale`, `unknown`, `not-run`, or infrastructure failure into
  `passed`; and
- does not waive a customer contract, law, legal hold, incident, notification duty, or
  independent auditor/regulator conclusion.

A customer obligation changes only through the approved contract amendment/waiver
process and authorized parties. A participant's liability waiver and release (previews,
on-site sessions, customer-hosted collection, events, exercises) never waives liability
the law does not allow to be waived and never replaces the MSA's limitation of liability.

## 7. Pull-request expectations

A PR affecting legal terms, document metadata, legal/audit code, customer entitlement,
retention/deletion, employment lifecycle, or generated legal projections should identify:

- affected stable document IDs and contract stack;
- whether the change affects risk allocation or legal effect;
- required legal/security/privacy/employment/business reviewers;
- generated artifacts and parity evidence that should change;
- migration/precedence implications for existing customers or workers; and
- why no executed, privileged, personal, secret, or customer-specific material is being
  committed.

Pull requests touching `docs/legal/`, `data/legal/`, `contracts/`, or the prices page
carry the `legal` label; price changes also carry `pricing` and a finance review. Do not
merge because a generated diff "looks right". Review the authored sources and the
resulting evidence independently.

## 8. Requests and sensitive facts

Use the `Legal / contract request` issue form only for public-safe routing metadata. Put
matter facts, party names, draft negotiations, employee/customer data, privileged advice,
settlement positions, signatures and evidence in the approved private legal/matter system.

Security vulnerabilities still follow `SECURITY.md`; do not open a public legal issue to
report a secret or vulnerability.

## 9. What must never be committed

Social Security or taxpayer identifiers, bank or card data, passport or identity scans,
individual home addresses, executed signature pages, real cap tables, customer names in
live statements of work, privileged legal advice, assessment working papers, customer
evidence, source secrets, access tokens, private keys, or production credentials — in any
repository of this organization, public or private.
