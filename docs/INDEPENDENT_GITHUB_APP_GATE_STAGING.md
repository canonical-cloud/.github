# Independent GitHub App taxonomy check — staging deployment contract

**Current status: implementation staged in draft PR #80, NOT DEPLOYED.** No GitHub App has been registered or installed by this PR. No protected check, ruleset or GitHub branch protection has been changed. The GitHub connection cannot create repositories or set organization rulesets.

## Trust boundary

The source in this draft PR is not a trusted authority. Copy the reviewed and pinned verifier source, `scripts/independent_taxonomy_gate.py`, `scripts/validate_repository_taxonomy.py`, `scripts/verify_taxonomy_review.py`, the schema, and tests into a **separate, administrator-protected code-only public verifier repository**. Verify SHA/content digests and provenance before publishing a protected release. Do not copy private registry records, production VPC identifiers, database credentials or App secrets.

The external App must execute only code and Python modules from that separately protected release. The candidate PR supplies JSON **data** obtained by GitHub REST at its exact commit SHA; it must never be able to supply executable code, a trusted schema, the expected workflow hash, the allowed organizations, or reviewer identities.

The independently required check is named `taxonomy-independent-gate`. Protect the **specific GitHub App issuer**, not just the string: a normal PR may be able to create a similarly named workflow check. A successful PR-owned workflow is not proof of enforcement.

## GitHub App registration (administrator)

- App repository permissions: **Contents: read**, **Pull requests: read**, **Checks: write**; GitHub includes Metadata: read. Do not request administration/secrets/write-to-contents.
- Install only in the pilot governance `.github` repositories. Configure signed pull-request events (opened, reopened, synchronize, ready-for-review, base edited) or a trusted repeatable poller for changes to protected PR heads; rerun after head changes. An event listener must verify GitHub `X-Hub-Signature-256` using its configured webhook secret and bind `installation.id`, repository ID, and PR number from that verified payload before minting a token.
- Run the evaluator on separately administered compute; **never** pass App private keys, webhook secrets, or installation tokens into the target repository's `pull_request` Actions jobs. An administrator's service should mint scoped 1-hour installation tokens and inject them only into the independent verifier execution. Do not log them.
- Enforce the App-produced check as a required, source-bound status check on each protected target branch; enable independent approvals/CODEOWNERS and restrict bypass roles. Verify effective rules with an administrator. Some private-repo ruleset features depend on GitHub plan.

GitHub App installation authentication follows [official documentation](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/authenticating-as-a-github-app-installation).

## Protected runtime contract

Current program entry point (only after independently protected promotion):

```sh
export INDEPENDENT_TAXONOMY_GATE_ENABLED=yes
export GITHUB_INSTALLATION_TOKEN="<injected only by the protected app runner>"
export GATE_TRUSTED_SCHEMA_PATH=/opt/protected-taxonomy/repository-taxonomy.schema.json
# No production promotions may be authorized yet:
export GATE_TRUSTED_REVIEWER_IDS=
python3 -m scripts.independent_taxonomy_gate --evaluate canonical-cloud/.github 76
```

The verified schema must be a regular **absolute, nonsymlinked file** matching Git blob `d2ec393f368d17793c6812b4867d3fedb50ef578`. The proposed taxonomy workflow must match pinned blob `9f8aed7672bf9e5021a61f8424268a550f738000`; a PR deleting or changing it receives a **failure** from the independent App. The checker reads real GitHub repository visibility, and the code-controlled list restricts evaluation to ten pilot organizations.

Its candidate JSON is fetched by GitHub API at the **specific PR head SHA**, read-only, with strict duplicate-key parsing and size bounds. Any missing API page, bad schema, changed workflow, unverified trust boundary or missing metadata fails closed. Check runs are attached to that exact SHA; if the PR head moves during evaluation, the old head cannot acquire a passing check for the new commit.

**Deliberate hard stop:** `ALLOW_REVIEWED_PROMOTIONS = False`. The App does not permit `review_status: reviewed` despite a complete GitHub review record. Before enabling any promotions, independently attest the actual IAM roles, VPC relationships, cloud deployment accounts and database access against trusted infrastructure evidence, require separate maintainer security approval, and perform new end-to-end adversarial tests.

## Evidence required before the pilot merges

- The check `taxonomy-independent-gate` appears on protected main-branch PRs **from the installed App identity** and is genuinely required by the branch/ruleset. Inspect its app/source identifier as an administrator.
- [Draft deletion test #77](https://github.com/canonical-cloud/.github/pull/77): removing the PR-owned workflow still triggers the independent gate, which reports **failure** and blocks an otherwise merge-eligible change.
- [Draft schema downgrade #78](https://github.com/canonical-cloud/.github/pull/78): independently rejected as a protected check failure.
- No reviewer can force, remove or spoof the App check or bypass required independent approval. Use a non-production protected sandbox to verify effective merge blocking, without merging malicious fixtures.
- Review the exact-head checks for ten pilot PRs, then merge with independently verified authorization and confirm main-branch execution and repository inventory reconciliation.

## Offline tests (no secrets, no network)

`python3 -m unittest discover -s tests -p 'test_independent_taxonomy_gate.py' -v`

The untrusted PR's Actions workflow runs mocked API tests with `contents: read` only. This local test workflow is **not** the independent gate and must not be required as a replacement for the App check.

Central tracking: [ORESoftware/.github #244](https://github.com/ORESoftware/.github/issues/244).
