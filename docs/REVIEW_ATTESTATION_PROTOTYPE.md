# Review attestation prototype (NOT an authorization gate)

This branch stages an offline-tested, **pure policy predicate** for independently verifying that a specific repository-taxonomy record is the exact content approved in a merged GitHub PR.

## Required trusted inputs

- The governance-repository PR object and review history must come from GitHub's authenticated API, fetched by an independently governed check.
- The approved repository-taxonomy row must be read at the **approved PR head commit SHA** from GitHub, not copied from the PR submitting a claim.
- `repo_full_name` and the allowed reviewer identities must come from an independently protected source of truth—not from the candidate repository manifest, PR labels, or PR-controlled workflow files.
- A reviewer cannot be the PR author; the latest review must be `APPROVED` at the exact PR head SHA; its timestamp must match the claim. The evidence PR must target `main`, be merged, belong to the expected governance repository, and contain **exactly the claimed row**.

## What this does not do

This prototype does not call GitHub, does not carry a production approval allowlist, and **cannot authorize any reviewed records**. Its only CLI entry point is `--self-test`, which uses synthetic mocked API responses. The installed production taxonomy verifier continues to fail closed for every `review_status: reviewed` entry.

A separately configured GitHub App, ruleset workflow, or equally independent trusted execution boundary must fetch the real PR and review evidence. Install review authorization and reviewer/branch protections first. The prototype may then be integrated after independent security review and real end-to-end acceptance tests.

Critically, a self-modifiable `pull_request` workflow with a passing check is not a required independent security gate. Do not treat this prototype's success as permission to merge the ten pilot PRs or promote privileged access.

## Tests

`python3 scripts/verify_taxonomy_review.py --self-test`

Tests reject forged timestamps and URLs, unmerged or cross-repository PRs, self-approval, stale or dismissed approvals, changed approved content, and unauthorized reviewer identities. CI runs the tests without network access or secrets.
