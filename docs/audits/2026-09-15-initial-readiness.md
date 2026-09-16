# Canonical Cloud initial compliance readiness self-audit

**Assessment date:** 2026-09-15  
**Scope of this pass:** GitHub organization/repository governance and the Canonical readiness/audit tooling that is directly observable through the current GitHub integration.  
**Framework references:** SOC 2 Trust Services Criteria, NIST CSF 2.0, NIST SP 800-53 Rev. 5, ISO/IEC 27001:2022.  
**Assurance statement:** This is an internal readiness artifact. It is not a SOC report, ISO certification, legal determination, or independent attestation.

## Executive summary

Canonical Cloud has a meaningful engineering-control foundation: organization security/governance documentation exists; the reusable GitHub Actions baseline enforces least-privilege workflow permissions, pinned actions, non-persistent checkout credentials, bounded jobs and several unsafe-pattern bans; the repository-hardening standard covers secrets, change control, CI/supply chain, negative testing, infrastructure and evidence integrity; and `canonical-company-auditor.rs` already separates deterministic evidence evaluation from narrative/AI output and recognizes missing evidence as `unknown`.

The present evidence is **not sufficient to claim SOC 2, NIST, or ISO compliance across the company**. The main reason is incomplete operating evidence outside the repository content itself: organization identity/access posture, periodic access reviews, branch-protection/classic-ruleset state, GitHub security-feature coverage, provider IAM/logging/security posture, risk/ISMS records, management/internal reviews, vendor evidence, incident/BCP exercises, and backup/restore results are not yet fully collected in this assessment boundary.

No inaccessible control is treated as failed merely because the GitHub integration or account plan does not expose its API.

## Evidence inspected

| Evidence | Observation |
| --- | --- |
| `canonical-cloud/.github/SECURITY.md` | Private vulnerability reporting and credential-rotation expectations are documented. |
| `.github/workflows/baseline-policy.yml` | Required org governance/architecture documents are validated; JSON and architecture authority checks run in CI. |
| `.github/workflows/reusable-policy.yml` | Explicit workflow permissions required; `write-all`, `pull_request_target`, inherited secrets and network-to-shell patterns rejected; external Actions pinned; checkout credentials disabled; runner timeouts enforced. |
| `docs/REPOSITORY_HARDENING_STANDARD.md` | Org standard covers contracts, Rust/app code, HTTP, persistence, infrastructure, CLIs, CI/supply chain, tests, documentation and exceptions. |
| `canonical-company-auditor.rs` | Deterministic readiness/audit/package engine exists with framework-reference overlays and pass/fail/unknown semantics. |
| public repository ruleset API | `.github`, `canonical-api-server.rs`, and `canonical-company-auditor.rs` returned no repository rulesets from the rulesets endpoint. This does not establish that classic branch protection is absent. |
| classic branch protection API | Current GitHub integration returned `403 Resource not accessible by integration`; status therefore remains unknown. |
| private `canonical-infra` ruleset API | Endpoint was unavailable under the current GitHub plan/API path; status therefore remains unknown. |
| `.canonical-cfg.toml` test fixture | Synthetic read-only provider inventory/secret-reference fixture is proposed in `canonical-cloud-test/canonical-api-server.rs#8`. |
| customer publication boundary | Private Git -> disclosure gate -> `customer/` -> Cloudflare R2 -> Access/signed-link model is proposed in `canonical-company-auditor.rs#19`. |

## Initial findings

### CC-2026-001 — Secure CI workflow baseline

**Status:** pass (design evidence)  
**Evidence quality:** repository policy + executable workflow checks  
**Observation:** The reusable organization workflow validates explicit permissions and rejects several high-risk workflow patterns. External Actions are required to use immutable commit SHAs and checkout credentials must not persist. Runner-backed jobs must have positive timeouts.

**Limit:** This pass has not yet proven that every production repository calls the reusable policy or an equivalent control. Fleet coverage must be measured before this can be treated as an organization-wide operating control.

**Framework relevance:** SOC 2 security/change-management criteria; NIST CSF Protect/Govern outcomes; NIST SP 800-53 change/configuration and least-privilege families; ISO/IEC 27001 ISMS/change/security engineering evidence.

### CC-2026-002 — Vulnerability reporting and credential exposure handling

**Status:** pass (design evidence)  
**Evidence quality:** organization policy  
**Observation:** The organization security policy directs suspected vulnerabilities to private reporting and requires exposed credentials to be revoked/rotated rather than merely deleted from a later commit.

**Limit:** Operating evidence for vulnerability intake, response timing, remediation samples and post-incident review has not yet been collected.

### CC-2026-003 — Repository hardening standard

**Status:** pass (design evidence)  
**Evidence quality:** organization standard  
**Observation:** The standard explicitly addresses secret boundaries, bounded inputs/timeouts, provider isolation, declarative migrations, least-privilege data access, SOPS/age, CI pinning, exact-head evidence, negative security tests and documented exceptions.

**Limit:** Fleet conformance has not yet been measured across every repository.

### CC-2026-004 — Protected default-branch evidence

**Status:** unknown  
**Evidence quality:** incomplete API visibility  
**Observation:** The repository-ruleset API returns no rulesets for several public critical repositories. The current GitHub integration cannot read classic branch-protection settings, and the private-infra ruleset endpoint is unavailable on the current API/plan path.

**Required evidence:** collect ruleset and/or classic branch-protection settings from an authorized organization-level source; document required reviews, status checks, force-push/deletion policy and justified exceptions.

### CC-2026-005 — Organization identity and periodic access governance

**Status:** unknown  
**Observation:** This pass does not contain authoritative evidence for organization 2FA enforcement, SSO/SCIM (where applicable), organization owners, outside collaborators, team membership, privileged GitHub Apps, deploy keys, service-account lifecycle, or periodic access reviews.

**Required evidence:** read-only organization identity/access export plus a dated reviewer-approved access-review record.

### CC-2026-006 — GitHub security-feature coverage

**Status:** unknown  
**Observation:** This pass has not yet produced a fleet matrix for secret scanning, push protection, code scanning, Dependabot alerts/updates, dependency review, private vulnerability reporting and security advisories.

**Required evidence:** collect these capabilities per repository and classify unsupported plan/API states separately from disabled controls.

### CC-2026-007 — Cloud and SaaS account security evidence

**Status:** unknown  
**Observation:** AWS, GCP, Azure, Cloudflare, Supabase, Neon, Vercel, Upstash, Kubernetes and observability provider posture has not yet been collected as part of this dated package.

**Required evidence:** `.canonical-cfg.toml`-driven read-only collectors for applicable accounts/projects covering IAM, audit logging, security controls, encryption/KMS, network exposure, resource health, monitoring and budgets/cost pressure.

### CC-2026-008 — ISMS and enterprise governance operating evidence

**Status:** unknown  
**Observation:** Repository policy evidence alone does not establish an operating information security management system. This pass does not yet contain the complete risk register/treatment plan, ISO Statement of Applicability, policy-owner review records, objectives/metrics, internal audit, management review, training/competence records, corrective-action records, or continual-improvement evidence.

**Required evidence:** gather dated governance artifacts and link them to evidence IDs rather than copying sensitive documents into customer-facing output.

### CC-2026-009 — Incident, continuity, backup and recovery operating tests

**Status:** unknown  
**Observation:** Written engineering policy references resilience and incident controls, but this package does not contain completed incident-response exercises, BCP/DR exercises, recovery objectives, backup inventories or successful restore-test evidence.

**Required evidence:** dated exercise/test records with owners, scope, results, exceptions and remediation follow-up.

### CC-2026-010 — Customer disclosure isolation

**Status:** unknown / implementation in progress  
**Observation:** The proposed architecture restricts publication to `customer/`, with path/symlink escape tests, secret scanning, integrity manifests and authenticated R2 delivery. The controls are documented but have not yet been proven against a live demo customer repository and Cloudflare endpoint.

**Required evidence:** create `customer-1-demo`, publish only `customer/`, then run negative HTTP tests proving `.canonical-cfg.toml`, `env/`, `evidence/`, `workpapers/`, Git metadata and traversal variants are unreachable.

## Framework-specific interpretation

### SOC 2

The current pass contains useful design evidence for security/change/supply-chain practices, but does not yet establish operating effectiveness across the assessment period. Availability, Processing Integrity, Confidentiality and Privacy must be separately scoped based on service commitments and system boundaries rather than assumed.

### NIST CSF 2.0

The existing policies and tooling support several Govern/Protect/Detect/Respond/Recover outcomes, but a complete Organizational Profile has not yet been produced from this evidence set. The next self-audit should export current-state and target-state profiles with evidence links and gaps.

### NIST SP 800-53 Rev. 5

Current tests can provide evidence for multiple control families, but mappings remain directional. The next package should record applicability, inheritance/shared responsibility and evidence freshness per selected control rather than presenting one technical check as satisfying an entire control.

### ISO/IEC 27001:2022

Technical hardening is only part of ISO readiness. The next package must include the management-system evidence needed for scope, risk assessment/treatment, applicability, objectives, competence/awareness, documented information, operational control, performance evaluation, internal audit, management review, corrective action and continual improvement, plus applicability review for Amendment 1:2024.

## Immediate remediation / next collection order

1. Land the compliance evidence standard and customer repository boundary after CI/review.
2. Make `.canonical-cfg.toml` a validated contract and implement explicit `--interactive` / `--non-interactive` behavior in the auditor CLI.
3. Create `customer-1-demo` as a private repository and copy the synthetic config/customer repository skeleton into it.
4. Collect GitHub organization/repository access, protection and security-feature evidence.
5. Collect applicable provider evidence through read-only identities.
6. Assemble risk/ISMS, vendor, incident, BCP/DR and restore-test operating evidence.
7. Generate separate SOC 2, NIST CSF, NIST 800-53 and ISO 27001 readiness outputs with evidence IDs and limitations.
8. Run the live Cloudflare publication negative tests before treating the customer portal control as implemented.

## Current conclusion

**Readiness state:** foundational controls present; significant operating-evidence gaps remain.  
**External compliance claim:** not supported by this evidence set yet.  
**Next gate:** close or explicitly accept the `unknown` items above with authoritative evidence and human review.
