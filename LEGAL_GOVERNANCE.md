# Canonical Cloud legal-document governance

This organization standard complements the detailed templates in the private
`canonical-cloud/canonical-docs` repository. It is governance and routing, not
legal advice and not an executed agreement. The full policy — ownership,
signing routes, required checks, waivers — is
[`policies/LEGAL_DOCUMENT_GOVERNANCE.md`](policies/LEGAL_DOCUMENT_GOVERNANCE.md).

## Contract stack

Customer engagements should identify the controlling stack explicitly rather
than copying clauses between files:

1. Master Services Agreement or other negotiated master agreement.
2. Order form / Statement of Work selecting scope, tier, price, deliverables,
   dependencies, acceptance, and change control.
3. Data Processing Agreement and security/privacy addenda where applicable.
4. Service Level Agreement only when contractual availability/service credits
   are purchased.
5. Support/maintenance, software license/EULA, audit-tool authorization, and
   other addenda only when applicable.
6. Internal SLOs remain engineering objectives and do not become customer
   warranties unless a signed agreement expressly says so.

The executed agreement controls over Markdown, website copy, generated data,
examples, linter output, or machine-readable receipts.

## Minimum maintained coverage

The legal corpus should maintain reviewed templates or playbooks for:

- MSA, SOW/order, change order, three-tier service schedule, SLA, and SLO;
- NDA (mutual and one-way), DPA, privacy terms, acceptable use, security terms,
  audit-tool authorization/limited waiver, and customer lifecycle
  (onboarding, offboarding and data return, termination and transition);
- software license/evaluation terms, EULA, support/maintenance, open-source and
  third-party license notices, and the participant liability waiver and
  release;
- W-2 employment, offer letter, contractor/advisor terms, confidentiality and
  invention assignment, workforce onboarding/offboarding, termination notice,
  and separation/release;
- corporate governance, founder/equity, board/stockholder consents, conflicts,
  and information security;
- independent-evaluator coordination with an explicit readiness-versus-
  assurance boundary; and
- a readiness engagement disclaimer (`canonical-docs`
  `docs/legal/external/readiness-engagement-disclaimer.md`) that every tier,
  pre-audit, training, and security-readiness offer references, and whose
  approved public summary backs the canonical.plus `/legal/` page: Canonical
  Cloud is not a CPA firm, law firm, certification body, or assessor, issues no
  audit opinions, attestations, certifications, or authorizations, and provides
  no legal advice.

Templates are not automatically safe for a real matter. Counsel must select and
localize the actual form and contract stack.

## Canonical service tiers

The schedule is governed data, not prose: `canonical-docs/data/legal/service-tiers.json`
is the only place a tier, price, term, support target, or SOW package changes,
and the drafting schedule `canonical-docs/docs/legal/external/service-tiers-and-pricing.md`,
the SLA and support-addendum tables, the SOW package menu, and the public
`/prices/` page are rendered from it (`python3 scripts/render_legal_data.py`).
The typed peer authorities live under `canonical-docs/contracts/`. The current
2026 planning/list snapshot (catalog `1.1.0`) is:

| Tier | Planning price | Minimum term | Managed operations | Bundled SOW package |
| --- | ---: | ---: | ---: | --- |
| Foundation | $4,000/month | 6 months | 6 hours/month | Readiness Diagnostic ($5,000 fixed) |
| Growth | $8,000/month | 6 months | 12 hours/month | SOC 2 Readiness Implementation ($18,000 fixed) |
| Scale | $12,000/month | 12 months | 20 hours/month | Multi-Framework Readiness Program ($45,000 fixed) |

This table is a snapshot for orientation; if it disagrees with the data file,
the data file is right and this table is stale. Prices remain planning
assumptions until a signed SOW/order creates an obligation.

## Docs ↔ code parity

For machine-readable legal/commercial data:

- TypeSpec and independently authored JSON Schema Draft 2020-12 are peer
  authorities.
- `ORESoftware/typespec-json-schema-validator` (`tjsv`) is the canonical
  fail-closed parity engine.
- `ORESoftware/ores-cli` (`oresc`) audits repository shape and invokes the same
  contract parity gateway; it does not become a third schema authority.
- Generated Markdown or website projections must be deterministic and checked
  for drift.
- The legal document registry is extracted from every template's metadata
  header (data from docs) and fails closed on an incomplete header.
- A green CI check never means counsel approved a legal document.

Use the reusable workflow in `.github/workflows/reusable-legal-contract-audit.yml`
for repositories that carry legal/commercial contracts.

## Required GitHub Actions configuration

`ores-cli` is private. Callers of the reusable legal workflow must provide a
least-privilege `ORES_TOOLING_READ_TOKEN` secret that can read only the required
ORESoftware tooling repository/repositories. Do not embed a token in a clone URL
or command line.

The marketing-site `/prices/` deployment uses browser-public Supabase values and
should configure them as repository/environment **variables**, not secrets:

- `CANONICAL_SUPABASE_URL`
- `CANONICAL_SUPABASE_PUBLISHABLE_KEY`
- `CANONICAL_PRICES_FUNCTION_URL` (when the authenticated prices function is
  deployed)

While the gate accepts a build-time one-time code, that code is the repository
**secret** `PRICES_OTP`; it is inlined into the `/prices/` bundle, so the
secret keeps it out of the repository and logs, not out of the published page.
Never publish a Supabase `service_role` or secret key to an Astro `PUBLIC_*`
variable or GitHub Pages artifact.

## Review gates

Changes to liability, indemnity, IP, privacy/data processing, employment,
waivers/releases, restrictive covenants, dispute terms, governing law, security
commitments, or assurance claims require qualified counsel review before use.
Pricing changes additionally require the commercial owner. SLA/SLO changes
require engineering/operations review. Public claims require substantiation and
must preserve the independent-assurance boundary.
