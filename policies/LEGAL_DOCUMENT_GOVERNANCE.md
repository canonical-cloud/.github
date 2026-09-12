# Legal document governance

> Organization-wide policy for how `canonical-cloud` drafts, governs, signs,
> and machine-checks its legal documents. It names which repository owns what,
> the signing route, and the checks every legal change must pass. It is not
> legal advice and it does not make any document an executed instrument.

## Ownership

| Concern | Authoritative home | Notes |
| --- | --- | --- |
| Legal templates (customer, partner, workforce, corporate) | [`canonical-docs`](https://github.com/canonical-cloud/canonical-docs) → `docs/legal/{external,internal}` | Markdown is the drafting source. Every template carries a metadata header with a `Document ID`, `Template version`, `Status`, `Owner`, `Review cadence`, and `Signing` row. |
| 170-type agreement catalog, playbooks, review gates | `canonical-docs` → `docs/legal/catalog/` | Catalog presence routes a matter to a playbook; it never asserts an execution-ready form. |
| Service tiers, prices, support targets, SOW packages | `canonical-docs` → `data/legal/service-tiers.json` | The only place a price changes. The pricing schedule, SLA and support tables, SOW package menu, and the public `/prices/` page are all rendered from it. |
| Legal data contracts (TypeSpec + JSON Schema peer authorities) | `canonical-docs` → `contracts/` | Compared by [`ORESoftware/typespec-json-schema-validator`](https://github.com/ORESoftware/typespec-json-schema-validator) (`tjsv`); audited by [`ORESoftware/ores-cli`](https://github.com/ORESoftware/ores-cli) (`oresc`). Neither authority is generated from the other. |
| Legal document registry (data extracted from the templates) | `canonical-docs` → `generated/legal-document-registry.json` | Consumed by this repository, the ores-legal signing platform, and `ores-cli`. |
| Public prices page | [`canonical-cloud.github.io`](https://github.com/canonical-cloud/canonical-cloud.github.io) → `/prices/` | Renders the synced copy `src/data/service-tiers.json`; gated by email + one-time code. Never edited by hand — refreshed from `canonical-docs`. |
| Executed instruments, signatures, acceptance records, digests | The governed records system and the [ores-legal](https://github.com/ores-legal) signing platform | **Never this organization's repositories.** No executed copy, signature page, filled placeholder, cap table, customer name in a live SOW, or personal identifier is committed anywhere. |
| Licensing of repository content | `canonical-docs` → `LICENSE-POLICY.md` | Intentional no-license-grant posture pending `DEN-628`. |

Linear: `DEN-1049` (legal documents), `DEN-628` (portfolio licensing).

## Signing routes

Every template declares exactly one route in its `Signing` header row. The
route decides what the executed record is; the Markdown is never it.

| Route | Meaning | Record |
| --- | --- | --- |
| `ores-legal-envelope` | Executed by the parties through an ores-legal envelope (MSA, SOW, SLA, DPA, NDAs, waivers, employment and separation documents, board consents). | Envelope identifier, completed-document digest, audit trail. |
| `counter-signed-order` | Incorporated by reference into a counter-signed Order Form (support addendum). | Order Form envelope and the referenced template version. |
| `acknowledgement` | Accepted by click-through or written acknowledgement (EULA, AUP, handbook, policies, checklists, audit-tooling terms). | Acceptance event with template version and time. |
| `not-signed` | Published, adopted, or filed elsewhere (ToS, privacy policy, SLOs, pricing schedule, charter documents, plans). | Adoption consent, filing receipt, or publication record. |

The ores-legal platform reads the registry's `signing` field to decide
whether a template is offered as an envelope template, an acknowledgement, or
not at all. A template whose header is incomplete does not reach the platform
because the registry extractor fails closed on it.

## Coverage the organization maintains

The corpus must keep a maintained template for at least: the Master Services
Agreement and Order Form; Statement of Work (with the bundled fixed-fee
packages); Service Level Agreement and Service Level Objectives; Support and
Maintenance Addendum; Data Processing Agreement; mutual and one-way NDAs;
Terms of Service, Acceptable Use Policy, and Privacy Policy; Software License
and Evaluation Agreement and End User License Agreement; Liability Waiver and
Release; Audit Tooling Terms and Authorization and the Continuous Readiness
Evidence Addendum; Independent Evaluator Coordination Agreement; Reseller and
Partner Agreement; Customer Onboarding Guide, Customer Offboarding and Data
Return, and Termination and Transition Agreement; and, for the workforce,
Offer Letter, Employment Agreement, Independent Contractor Agreement, CIIAA,
Advisor Agreement, Employee Handbook, onboarding and offboarding checklists,
Engagement Termination Notice, and Separation Agreement and General Release;
plus the formation, equity, and policy set. `canonical-docs/scripts/check_legal.py`
fails when any required template is missing.

## Non-reliance and claims

Canonical Cloud does not issue SOC reports, ISO certificates, FedRAMP
authorizations, PCI attestations, audit opinions, or other independent
assurance, and no template, page, or tool output may say or imply that it
does. Prices are planning figures until validated through paid engagements;
a price on a page or in a file is not an offer. The `claims-register.md` in
`canonical-docs` governs public language, and the legal audit rejects
certification, guarantee, and absolute-availability claims.

## Required checks

A change to any legal template, catalog, data file, contract authority, or
the public prices page passes all of the following before merge:

1. `python3 scripts/check_legal.py` — leaks, claims, required clauses, header
   hygiene, index integrity.
2. `python3 scripts/check_legal_catalog.py` — catalog structure and the
   readiness boundary phrases.
3. `python3 scripts/check_legal_data.py`, `render_legal_data.py --check`,
   `extract_legal_registry.py --check` — data satisfies the authored contract,
   documents match the data, registry matches the documents.
4. The `tjsv` peer-authority gate (pinned action) over `contracts/` with the
   instance corpus, emitting a digest-bound Contract IR.
5. `oresc audit repo --profile docs` and `oresc audit contract`, with any
   accepted finding recorded in `ops/ores-cli-accepted-findings.json` naming
   its governing decision.
6. For the site: `npm test` and `npm run build` in `canonical-cloud.github.io`,
   which fail if any page other than `/prices/` references Supabase or the
   gate variables, or if the built page disagrees with the synced catalog.

The selectable workflow template
[`legal-corpus-audit`](../workflow-templates/legal-corpus-audit.yml) installs
checks 1–4 in any repository that carries a `docs/legal/` tree.

## Change control

- Counsel review is required before any template is used, and before merge
  for changes to liability, indemnity, IP, data protection, employment
  restrictions, consumer rights, releases, or regulatory language.
- Template versions follow semver: patch for wording, minor for a
  clarification or new optional clause, major for changed risk allocation or
  legal effect. The catalog version and the pricing catalog version follow
  the same rule.
- Executed versions are never overwritten. Amendments, renewals, assignments,
  terminations, and corrections are new linked records in the records
  system.
- Pull requests touching `docs/legal/`, `data/legal/`, `contracts/`, or the
  prices page carry the `legal` label; price changes also carry `pricing`
  and a finance review.

## What must never be committed

Social Security or taxpayer identifiers, bank or card data, passport or
identity scans, individual home addresses, executed signature pages, real cap
tables, customer names in live statements of work, privileged legal advice,
assessment working papers, customer evidence, source secrets, access tokens,
private keys, or production credentials — in any repository of this
organization, public or private.
