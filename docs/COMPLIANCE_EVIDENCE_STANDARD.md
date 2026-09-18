# Canonical Cloud compliance evidence standard

This document defines the evidence discipline Canonical Cloud uses for internal readiness reviews and customer audit engagements. It is intentionally framework-neutral at the evidence layer and supports directional mappings to SOC 2 Trust Services Criteria, NIST CSF 2.0, NIST SP 800-53 Rev. 5, and ISO/IEC 27001:2022.

It does **not** claim that Canonical Cloud is certified, attested, or fully compliant. Those conclusions require applicable scope, implemented controls, operating evidence, and where relevant an independent qualified auditor or certification body.

## 1. Evidence principles

Every compliance assertion must be backed by evidence that is:

- scoped to a named organization, system, environment, account/project, and assessment period;
- collected read-only unless an explicit approved remediation action is being performed separately;
- provenance-bearing: source system, collector version, collection time, source commit/program version, and stable evidence identifier;
- content-addressed or otherwise integrity-verifiable where practical;
- classified for internal, auditor-only, customer-visible, or public disclosure;
- redacted so reports never contain bearer credentials, private keys, database passwords, session tokens, or unrestricted signed URLs;
- retained according to the applicable evidence-retention policy; and
- reviewed by a human control owner before an external compliance claim is made.

`unknown` is a first-class result. Missing or stale evidence is not converted into a pass and is not automatically treated as proof that the control failed.

## 2. Self-audit scope

The Canonical Cloud self-audit must cover at least these control families:

| Control family | Required evidence examples |
| --- | --- |
| Governance and ISMS | approved policies, owners, review dates, scope, risk register, treatment plan, applicability decisions, management review, internal audit |
| Identity and access | workforce/role inventory, MFA/SSO posture, privileged access, onboarding/offboarding samples, periodic access reviews, service-account ownership |
| Source and change management | repository inventory, protected change paths, review evidence, exact-head CI, release provenance, deployment approvals, rollback evidence |
| Supply chain | dependency inventories, pinned CI actions, advisory handling, SBOM/provenance where applicable, third-party action/app review |
| Secrets and cryptography | SOPS/age or workload identity, secret scanning, rotation evidence, KMS/key ownership, encryption in transit/at rest |
| Infrastructure and network | cloud account inventory, IAM, logging, firewall/WAF/network policy, compute/storage/database posture, configuration drift |
| Detection and response | audit/security logs, alert routing, incident plan, exercise/tabletop evidence, incident records and corrective actions |
| Vulnerability management | scanning coverage, remediation/patch SLAs, exceptions, verification and aging |
| Availability and resilience | service objectives, backup policy, restore tests, DR/BCP exercises, capacity/resource monitoring |
| Data and privacy | data inventory/classification, retention/deletion, access controls, processor/vendor records, privacy applicability and request handling |
| Vendor risk | vendor inventory, due diligence, contractual safeguards, continuing monitoring and exit/termination controls |
| Customer disclosure | publication allowlist, integrity manifest, authorization boundary, negative tests proving private evidence/workpapers are not exposed |

## 3. GitHub control evidence

For every production or security-sensitive repository, collect or explicitly mark unavailable/not-applicable evidence for:

- repository visibility and archival state;
- default branch and protected-change mechanism (ruleset and/or classic branch protection);
- required reviews and status checks;
- CODEOWNERS/ownership where used;
- Actions default token permissions and workflow-level permissions;
- pinned external Actions and checkout credential persistence policy;
- allowed Actions/reusable workflow policy;
- environments, reviewers, deployment branches and OIDC/cloud identity usage;
- Dependabot/dependency review state;
- code scanning state;
- secret scanning and push protection state;
- private vulnerability reporting/security advisory path;
- deploy keys, GitHub Apps and outside collaborator exposure where the account API allows collection;
- release/tag protection and artifact provenance where applicable.

If an API endpoint is inaccessible because of GitHub plan, application permissions, or repository visibility, record the limitation as an evidence gap. Do not infer a disabled control from a 403 or missing API permission.

## 4. Cloud/provider evidence

`.canonical-cfg.toml` is the inventory of service connections and audit scope. It must contain only non-secret connection metadata plus references to secret material. Provider collectors must request the narrowest practical read-only roles.

Minimum provider families for Canonical Cloud's own audit:

- GitHub;
- Cloudflare;
- AWS, GCP and Azure where accounts/resources exist;
- Supabase and Neon;
- Vercel where projects exist;
- Upstash/Redis where used;
- Kubernetes clusters/contexts where used;
- observability providers such as Sentry/Datadog where used.

For each provider, collect identity/IAM, audit logging, security controls, encryption/KMS, networking, compute/storage/database inventory, monitoring/alerts, and cost/budget/resource-pressure signals as applicable. Unsupported checks are reported as unsupported/unknown, not silently omitted.

## 5. Framework handling

### SOC 2

SOC 2 readiness is evaluated against the applicable Trust Services Criteria. Security is the common baseline; Availability, Processing Integrity, Confidentiality, and Privacy are scoped based on the services and commitments being assessed. The self-audit must preserve the distinction between readiness testing and an independent CPA attestation.

### NIST CSF 2.0

Evidence should be organized so it can be profiled against the CSF 2.0 functions and outcomes. CSF results are risk-management outcomes, not a certification score.

### NIST SP 800-53 Rev. 5

Mappings are control-reference aids. Scope and applicability must be tailored; one Canonical technical check may support multiple controls and must not be presented as satisfying an entire control family by itself.

### ISO/IEC 27001:2022

The self-audit must cover the ISMS requirements in addition to technical controls. Maintain the ISMS scope, risk assessment/treatment process, Statement of Applicability, objectives, competence/awareness, documented information, operational planning, performance evaluation, internal audit, management review, nonconformity/corrective action and continual-improvement evidence. Review the applicability of Amendment 1:2024 in the management-system context.

## 6. Customer audit repository

Each customer engagement uses one private repository with these boundaries:

```text
.canonical-cfg.toml
source/config metadata

evidence/       private normalized evidence
workpapers/     private auditor/reviewer work
reports/        complete generated internal reports
manifests/      integrity/provenance records
customer/       only publishable subtree
```

Live credentials are never committed. Use SOPS+age under the approved `env/enc` boundary or an external secret store and keep the TOML limited to references such as `env:`, `sops:`, `file:` or `vault:`.

## 7. Customer publication controls

The customer-facing path is published independently from Git history. Preferred architecture for authenticated documents:

```text
private Git repo -> CI disclosure gate -> customer/ -> Cloudflare R2 -> custom domain -> Access/signed-link policy
```

The publisher must:

- resolve and allowlist every path beneath `customer/`;
- reject symlink/path traversal escapes;
- scan for secrets before upload;
- attach expected MIME/content-disposition/security headers;
- generate a publication manifest with source commit, object bytes and SHA-256;
- have write permission only to the intended bucket/prefix and no routine bucket-admin permission;
- keep production `r2.dev` access disabled;
- run post-publication negative tests against private paths; and
- preserve publication evidence in Git.

## 8. Review cadence

At minimum:

- automated technical evidence: continuously/on change and during each formal assessment window;
- access review: on the documented periodic cadence and after material role changes;
- risk register/treatment plan: after material change and on the management-review cadence;
- policies: on their documented review cadence and after material legal/technical changes;
- incident/BCP/restore exercises: on the documented testing cadence;
- vendor reviews: before onboarding and periodically based on risk tier;
- full framework readiness package: before external audit/certification activity and after material scope changes.

## 9. Output requirements

Every readiness package must identify:

- assessment scope and period;
- program/framework versions;
- evidence collection limitations;
- `pass`, `fail`, `unknown`, and not-applicable outcomes without collapsing them;
- findings, severity/risk rationale and remediation owner/status;
- source/program/evidence digests where supported;
- framework crosswalks with an explicit non-equivalence disclaimer; and
- the reviewer/approval state for any externally disclosed version.

Tracking issue: `canonical-cloud/.github#67`.
