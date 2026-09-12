# Canonical Cloud legal-document governance

This organization standard complements the detailed templates in the private
`canonical-cloud/canonical-docs` repository. It is governance and routing, not
legal advice and not an executed agreement.

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
  audit-tool authorization/limited waiver, and customer lifecycle;
- software license/evaluation terms, EULA, support/maintenance, open-source and
  third-party license notices;
- W-2 employment, offer letter, contractor/advisor terms, confidentiality and
  invention assignment, workforce onboarding/offboarding, termination notice,
  and separation/release;
- corporate governance, founder/equity, board/stockholder consents, conflicts,
  and information security; and
- independent-evaluator coordination with an explicit readiness-versus-
  assurance boundary.

Templates are not automatically safe for a real matter. Counsel must select and
localize the actual form and contract stack.

## Canonical service tiers

The current 2026 planning/list schedule is:

| Tier | Planning price | Minimum term |
| --- | ---: | ---: |
| Foundation | $4,000/month | 6 months |
| Growth | $8,000/month | 6 months |
| Scale | $12,000/month | 12 months |

The authoritative drafting schedule is
`canonical-docs/docs/legal/external/service-tiers-and-pricing.md`; the typed
instance lives under `canonical-docs/contracts/service-tiers/`. Prices remain
planning assumptions until a signed SOW/order creates an obligation.

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
- `CANONICAL_PRICES_FUNCTION_URL`

Never publish a Supabase `service_role` or secret key to an Astro `PUBLIC_*`
variable or GitHub Pages artifact.

## Review gates

Changes to liability, indemnity, IP, privacy/data processing, employment,
waivers/releases, restrictive covenants, dispute terms, governing law, security
commitments, or assurance claims require qualified counsel review before use.
Pricing changes additionally require the commercial owner. SLA/SLO changes
require engineering/operations review. Public claims require substantiation and
must preserve the independent-assurance boundary.
