# Legal and contract routing for Canonical Cloud repositories

> **PUBLIC-SAFE ROUTING GUIDE — NOT LEGAL ADVICE**

The detailed drafting corpus is private in `canonical-cloud/canonical-docs`. This page
helps repository maintainers recognize when a code/docs change crosses a legal boundary.
Do not paste private contract language or matter facts into public repositories.

| Change | Primary legal surface | Additional review |
| --- | --- | --- |
| New customer product / subscription / quote | MSA, order form, SOW, ToS | Finance, tax, product |
| Installed or downloadable software | EULA / software license / OSS notices | Product, security, OSS/IP |
| API or developer platform | MSA/ToS, API/license terms | Product, security, privacy |
| Customer data processing | DPA, privacy policy, security terms | Privacy, security |
| SLA/support change | SLA, support addendum, order form | SRE/support, finance |
| Customer-specific exception | Contract waiver/exception addendum | Counsel + affected control owner |
| Customer termination/offboarding | Termination agreement, DPA/retention, transition plan | Legal, privacy, security, finance |
| NDA/confidentiality | Mutual or unilateral NDA | Legal/business owner |
| Hire or contractor onboarding | Offer/employment/contractor agreement, CIIAA, handbook | Employment counsel, HR, security |
| Worker separation | Termination notice, separation/release if used | Employment counsel, HR, payroll/benefits |
| Worker offboarding | Offboarding checklist, retention/legal hold | HR, security, privacy/legal |
| Audit/lint suppression | Internal audit exception/waiver policy | Control owner; specialist review by risk |
| TypeSpec/schema legal metadata | `contracts/legal/*` plus registry | Legal + engineering |
| Generated legal IDs/reference docs | `generated/legal/*` | Engineering; regenerate only |
| Retention/deletion behavior | DPA/privacy/records/legal hold | Privacy, legal, security |
| Auth or tenant authority change | Customer terms/security + system contracts | Security, legal/product |
| Marketing compliance claim | Legal/compliance claim review | Legal + assurance owner |

## Repository triggers

Escalate a PR for legal review when it changes anything that can alter who is bound, what
a customer/worker may do, price/renewal/termination, data use/retention/deletion, IP or
license rights, confidentiality, warranty/indemnity/liability, audit rights, employment
rights, waiver/exception behavior, or the machine-readable IDs/status that product code
uses to select legal documents.

## Machine evidence

Canonical uses TJSV to compare independently authored TypeSpec and Draft 2020-12 JSON
Schema for selected legal metadata. Rust/TypeScript/reference projections are generated
only after that boundary is defined. `ores-cli` audits repository/config/source structure
where its private-source CI credential is provisioned.

None of these tools interprets law or approves legal language. A legal approval record is
separate evidence bound to the exact document revision/digest and review scope.

## Where not to put matter data

Do not put party names, addresses, signatures, tax IDs, employment records, customer
secrets, settlement terms, legal advice, security vulnerabilities, raw audit evidence, or
negotiation history into public GitHub issues, PR descriptions, generated receipts, or
source comments. Use stable matter/document IDs and controlled systems instead.
