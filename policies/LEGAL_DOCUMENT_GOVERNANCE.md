# Canonical Cloud legal document governance

> **PUBLIC-SAFE GOVERNANCE — NOT LEGAL ADVICE — NOT AN EXECUTED AGREEMENT**

Canonical Cloud keeps detailed legal drafting in the private `canonical-cloud/canonical-docs`
repository. This public organization repository contains policy and routing only. Do not
copy executed contracts, customer confidential information, employee records, privileged
legal advice, signatures, identity documents, financial data, secrets, or regulated
customer evidence here.

## 1. Source boundaries

- `canonical-cloud/canonical-docs` owns Canonical Cloud legal templates, clause playbooks,
  legal-document metadata contracts, and legal audit tooling.
- `ores-legal/*` supplies reusable signing/evidence architecture and patterns; Canonical
  documents remain Canonical-specific and require their own counsel approval.
- Product/code repositories may reference stable legal document IDs and approved versions
  but should not fork legal prose into application source.
- Executed agreements belong in the governed records system, not Git.

## 2. Matter coverage

The legal program routes at least these matter classes:

- customer MSA/SOW/order forms, SaaS/cloud/service terms, SLA/support, amendments,
  renewals, termination and transition;
- privacy policy, DPA, security/privacy addenda, subprocessors, data sharing/licensing,
  retention/deletion and customer evidence terms;
- EULA/software/evaluation/API/open-source and other IP licensing;
- mutual and unilateral NDAs, confidentiality, invention assignment and IP ownership;
- customer contract exceptions, waivers, credits and negotiated deviations;
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

## 3. Review and approval

Legal language cannot be approved by AI, CI, a linter, a schema validator, a generated
receipt, a project-management state, or a source-control merge. Qualified counsel must
approve the exact matter-specific version when legal review is required. Security,
privacy, finance/tax, HR/employment, product, and business owners review their respective
facts and controls.

Approval evidence should bind the exact source revision/content digest, document ID and
version, reviewer role, jurisdiction/matter scope, decision reference, and decision time.
A later substantive edit invalidates stale approval.

## 4. Docs ↔ code synchronization

For selected machine-readable legal metadata—not clause wording—Canonical Cloud uses:

- independently authored TypeSpec;
- independently authored JSON Schema Draft 2020-12;
- pinned `ORESoftware/typespec-json-schema-validator` parity and Contract IR evidence;
- deterministic Rust/TypeScript/reference-document projections; and
- `ORESoftware/ores-cli` repository/config/source governance when the private tool is
  provisioned in the CI trust boundary.

Generated output is derivative and must never overwrite either authored machine authority
or counsel-owned legal prose. A parity receipt proves only the configured machine
contract converged for the exact checked source.

## 5. Waivers and exceptions

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
process and authorized parties.

## 6. Pull-request expectations

A PR affecting legal terms, document metadata, legal/audit code, customer entitlement,
retention/deletion, employment lifecycle, or generated legal projections should identify:

- affected stable document IDs and contract stack;
- whether the change affects risk allocation or legal effect;
- required legal/security/privacy/employment/business reviewers;
- generated artifacts and parity evidence that should change;
- migration/precedence implications for existing customers or workers; and
- why no executed, privileged, personal, secret, or customer-specific material is being
  committed.

Do not merge because a generated diff "looks right". Review the authored sources and the
resulting evidence independently.

## 7. Requests and sensitive facts

Use the `Legal / contract request` issue form only for public-safe routing metadata. Put
matter facts, party names, draft negotiations, employee/customer data, privileged advice,
settlement positions, signatures and evidence in the approved private legal/matter system.

Security vulnerabilities still follow `SECURITY.md`; do not open a public legal issue to
report a secret or vulnerability.
