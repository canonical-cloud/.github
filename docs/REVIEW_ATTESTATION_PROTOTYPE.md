# Review attestation prototype — NOT an authorization gate

This branch implements a **pure, offline-testable policy predicate** for a future independently governed GitHub approval gate. It does not fetch GitHub records and it cannot turn on any privileged repository-taxonomy classification.

## Two-phase, exact-head approval design

A separate governance PR proposes the **precise intended reviewed policy record**, stored at a reviewed proposal path such as `review-proposals/example-admin-api-server.json`. That proposal must **exclude** `review_evidence`, because the actual review timestamp is not known until the reviewer approves it. After the proposal PR merges, the candidate registry record may attach `review_evidence` without altering any of the approved substantive fields.

The independently protected evaluator, not the candidate, must retrieve:

- The proposed record blob and its origin (governance repo, fixed proposal path, and **final PR head commit SHA**) from authenticated GitHub APIs.
- The canonical merged PR object, **complete paginated review history**, and the final revision SHA.
- An independently protected **allowlist of stable numeric GitHub user IDs** (not mutable account logins). A trusted reviewer must differ from the PR author by stable ID.
- Verified IaC/identity/database evidence for deployment and access claims. This prototype checks review provenance only; it **does not** validate infrastructure facts.

The predicate rejects a claim unless the proposed record exactly equals the candidate with only `review_evidence` removed. It requires a merged governance PR targeting `main`, the correct GitHub URL, reviewer ID in the protected allowlist, current `APPROVED` review from that reviewer on the exact final head SHA, a matching timestamp, and complete evidence. When review timestamps are equal, GitHub review IDs break ties deterministically; later dismissals/changes requests reject the claim.

A boolean indicating review-history completeness is **not itself proof**: only a trusted API caller can provide it after fetching all pages. Similarly, the proposal-origin mapping cannot be trusted if supplied by the PR author.

## Production remains fail closed

The module CLI exposes **only** `--self-test`; invoking it without that flag must fail. The production repository-taxonomy verifier continues to reject every `review_status: reviewed` claim until the protected evaluator and administrator-owned reviewer authorization are installed and independently audited. This branch must not be used as a required check or a source of authorization.

The local test workflow executes candidate-branch code for development feedback; because the PR author can modify that code, **passing its tests is not independent enforcement**.

## Verification

Run `python3 scripts/verify_taxonomy_review.py --self-test` to exercise the 40 positive and negative fixtures, including stable-ID spoofing, self-review, reordered/dismissed reviews, duplicated review IDs, missing evidence pages, mismatched proposal origins and contents, and forged timestamps. CI additionally verifies that the production command without `--self-test` exits unsuccessfully.

See [INDEPENDENT_TAXONOMY_GATE_RUNBOOK.md](./INDEPENDENT_TAXONOMY_GATE_RUNBOOK.md) for the administrator-owned deployment and adversarial acceptance steps.
