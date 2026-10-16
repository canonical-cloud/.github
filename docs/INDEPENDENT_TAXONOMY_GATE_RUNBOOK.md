# Independent repository-taxonomy gate: administrator runbook

**Status: prepared, not deployed.** This pilot cannot yet block merges independently of the PR-owned workflow.

## 1. Provision an independent public verifier home

Create a dedicated **public code-only** repository (suggested: `ORESoftware/repository-taxonomy-verifier`), owned and protected by a separate governance maintenance group. Do **not** migrate private organization inventory, VPC identifiers, IAM identities, registry contents, or production credentials into it.

Promote the already tested source from canonical-cloud/.github, pin verifier and schema to immutable commit SHA and verified content hashes. Configure branch/tag protection and signed/reviewed releases as supported. Keep the old pinned revision reachable during cutover. Do not source executable verifier code from the candidate PR branch.

## 2. Choose an independently required execution model

Preferred: a separately governed GitHub App reporting a unique check such as `taxonomy-independent-gate`, configured as a **required status check from that exact app** on every protected target branch. Alternative: a genuinely independent organization/enterprise ruleset workflow on eligible GitHub plans, administered outside the candidate repositories.

A PR-owned `pull_request` workflow that reports `taxonomy-admission` **is insufficient** even with pinned verifier code: deleting or editing its workflow file can stop or alter execution. Merely requiring a check name without controlling its issuer and execution provenance may also be insufficient. Protect ruleset administration and bypass eligibility.

For any GitHub App executor, use read-only repository and PR-review access, plus only the minimum check-publication permission required. Separate execution identity from application deployment identities. Never execute untrusted PR-sourced Python, shell, Makefiles or dependencies in a privileged runner.

## 3. Inputs and review provenance

The independent gate should read the exact PR head commit and registry/schema **as data** using GitHub APIs, fetch the approved verifier from the protected authority, and compare proposed schema to authoritative pinned schema. It must fail on unavailable/malformed API data and missing complete inventory evidence rather than silently treating a partial scan as complete.

Before promoting any `review_status: reviewed`, integrate `scripts/verify_taxonomy_review.py` as a pure predicate with GitHub-API-sourced, trustworthy PR and review data, the exact record at the approved head commit, and an independently protected list of eligible reviewers. Enforce distinct PR author and approver, current APPROVED review of the final head SHA, merged governance PR, and exact record match. The prototype currently offers **offline tests only**, so **retain the production deny-all** until the authenticated caller and administrator-maintained reviewer authority pass external review.

Review Terraform/IaC and cloud evidence for actual admin-web/admin-API/admin-MCP VPC and database boundaries. Registry names are never authorization.

## 4. Real adversarial acceptance tests

- [ ] [Draft PR #77 — workflow deletion](https://github.com/canonical-cloud/.github/pull/77) produces **no PR-owned taxonomy workflow run**, while unrelated workflows pass. The independent gate must still execute and **block this specific deletion** on a protected target branch. A draft PR's intrinsic inability to merge is not proof of enforcement. Repeat the test with an independently approved, non-draft, non-mergeable **sandbox target branch**, or equivalent administrator-visible ruleset evaluation without merging malicious content.
- [ ] [Draft PR #78 — schema weakening](https://github.com/canonical-cloud/.github/pull/78) is rejected (the current pinned gate already fails with a schema-difference error). The independent required gate must also reject it.
- [ ] A forged `review_evidence.url`, self-approval, dismissed review, stale review SHA, and changed reviewed record are denied.
- [ ] A fully authorized approved record is accepted **only** by the protected gate, after exact-head attestation and release/VPC/DB evidence checks.
- [ ] Attempt to remove or modify the independent gate from a normal PR still results in an externally required failure/missing check; a contributor cannot spoof the required check source.
- [ ] Mandatory checks and independent review are effective for the target main branches, with no unconstrained bypass role.
- [ ] All ten pilot PRs have up-to-date passing checks at merged head and on main after independently reviewed merge.

## 5. Plan and permission blockers

The current ChatGPT GitHub connector provides repository file/PR/issue operations but **not repository creation or GitHub ruleset administration writes**. A canonical-cloud ruleset read returned no rulesets, and branch-protection GET was forbidden to the connected GitHub App; therefore effective branch protections could not be fully verified from this connection. Private repositories may need a plan-compatible external App check or additional GitHub tier.

Tracking issue: [ORESoftware/.github #244](https://github.com/ORESoftware/.github/issues/244). All adversarial PRs are testing fixtures; **do not merge**.
