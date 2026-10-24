#!/usr/bin/env python3
"""Offline review-attestation policy prototype. NOT a production authorization gate.

A separately protected GitHub App/ruleset must fetch ALL trusted inputs.
No CLI accepts candidate policy data or an approver allowlist. In production,
the repository-taxonomy admission checker still DENIES every reviewed record.
"""
import argparse
import copy
import datetime
import re

UTC = datetime.timezone.utc
SHA = re.compile(r"[0-9a-f]{40}\Z")
LOGIN = re.compile(r"[A-Za-z0-9-]+\Z")
REPO = re.compile(r"[A-Za-z0-9_.-]+/\.github\Z")
PROPOSAL = re.compile(r"review-proposals/[A-Za-z0-9_.-]+\.json\Z")


class ReviewDenied(ValueError):
    pass


def deny(message):
    raise ReviewDenied(message)


def utc_time(value):
    if not isinstance(value, str):
        deny("timestamp is not a string")
    try:
        result = datetime.datetime.fromisoformat(value.replace("Z", "+00:00"))
    except (ValueError, OverflowError):
        deny("invalid timestamp")
    if result.tzinfo is None or result.utcoffset() is None:
        deny("timestamp missing timezone")
    return result.astimezone(UTC)


def positive_int(value):
    return type(value) is int and value > 0


def verify_reviewed_row(
    row, pr, reviews, approved_proposal, proposal_origin, *,
    repo_full_name, trusted_reviewer_ids, review_history_complete,
):
    """Pure predicate on authenticated, complete GitHub API evidence.

    The only candidate-authored input is 'row'. The remaining inputs MUST come
    from a protected caller with an authenticated GitHub App and an independently
    administered reviewer-ID allowlist; this function cannot establish provenance.

    approved_proposal: exact proposed reviewed row *without* review_evidence,
       stored as a separate approval proposal before review. The protected
       caller retrieves its Git blob at PR head SHA, never from the claim.
    proposal_origin: protected caller's authenticated repository/path/commit
       information for that blob, not a claim supplied by the candidate PR.
    """
    if not isinstance(repo_full_name, str) or not REPO.fullmatch(repo_full_name):
        deny("invalid governance repository")
    if type(review_history_complete) is not bool or not review_history_complete:
        deny("review history not known complete")
    if type(trusted_reviewer_ids) not in (set, frozenset) or not trusted_reviewer_ids:
        deny("trusted stable reviewer IDs unavailable")
    if not all(positive_int(x) for x in trusted_reviewer_ids):
        deny("trusted IDs must be positive GitHub numeric user IDs")
    if not isinstance(row, dict) or row.get("review_status") != "reviewed":
        deny("candidate has no reviewed classification")
    evidence = row.get("review_evidence")
    if not isinstance(evidence, dict) or set(evidence) != {"url", "reviewed_by", "reviewed_at"}:
        deny("malformed review evidence")
    claimed_login = evidence["reviewed_by"]
    if not isinstance(claimed_login, str) or not LOGIN.fullmatch(claimed_login):
        deny("invalid claimed reviewer login")
    if not isinstance(approved_proposal, dict) or "review_evidence" in approved_proposal:
        deny("untrusted or malformed approval proposal")
    if approved_proposal.get("review_status") != "reviewed":
        deny("proposal must explicitly request reviewed classification")
    without_evidence = {key: value for key, value in row.items() if key != "review_evidence"}
    if without_evidence != approved_proposal:
        deny("candidate differs from independently approved proposal")
    if not isinstance(pr, dict) or not isinstance(reviews, list):
        deny("authenticated PR or complete reviews unavailable")
    if pr.get("merged") is not True or pr.get("state") != "closed":
        deny("approval PR is not merged")
    if not positive_int(pr.get("number")):
        deny("invalid PR number")
    base = pr.get("base")
    if (not isinstance(base, dict) or base.get("ref") != "main"
            or not isinstance(base.get("repo"), dict)
            or base["repo"].get("full_name") != repo_full_name):
        deny("approval PR not merged into expected main repository")
    head = pr.get("head")
    if not isinstance(head, dict):
        deny("approval PR head missing")
    head_sha = head.get("sha")
    if not isinstance(head_sha, str) or not SHA.fullmatch(head_sha):
        deny("invalid approved PR commit SHA")
    if not isinstance(proposal_origin, dict) or set(proposal_origin) != {"repository", "commit_sha", "path"}:
        deny("authenticated proposal origin missing")
    if (proposal_origin["repository"] != repo_full_name or
            proposal_origin["commit_sha"] != head_sha or
            not isinstance(proposal_origin["path"], str) or
            not PROPOSAL.fullmatch(proposal_origin["path"])):
        deny("approval proposal not fetched at exact final PR head")
    canonical_url = f"https://github.com/{repo_full_name}/pull/{pr['number']}"
    if evidence["url"] != canonical_url or pr.get("html_url") != canonical_url:
        deny("claim points at incorrect PR URL")
    author = pr.get("user")
    if not isinstance(author, dict) or not positive_int(author.get("id")):
        deny("PR author stable identity unavailable")
    claim_time = utc_time(evidence["reviewed_at"])
    merged_time = utc_time(pr.get("merged_at"))
    if merged_time < claim_time or claim_time > datetime.datetime.now(UTC) + datetime.timedelta(minutes=5):
        deny("approval timestamp outside merge/future bounds")
    seen_review_ids = set()
    events_by_reviewer_id = {}
    for review in reviews:
        if not isinstance(review, dict) or not isinstance(review.get("user"), dict):
            deny("invalid review history item")
        user = review["user"]
        if (not positive_int(review.get("id")) or review["id"] in seen_review_ids
                or not positive_int(user.get("id"))
                or not isinstance(user.get("login"), str)):
            deny("ambiguous review ID or reviewer identity")
        seen_review_ids.add(review["id"])
        submitted = utc_time(review.get("submitted_at"))
        if submitted > merged_time + datetime.timedelta(minutes=5):
            deny("review history contains post-merge event")
        if user["id"] in trusted_reviewer_ids and user["id"] != author["id"]:
            events_by_reviewer_id.setdefault(user["id"], []).append(
                (submitted, review["id"], review))
    candidates = []
    for user_id, events in events_by_reviewer_id.items():
        # Process every review under the stable numeric ID, even if the account
        # login changed between reviews. Review IDs resolve timestamp ties.
        events.sort(key=lambda event: (event[0], event[1]))
        last_time, _, last = events[-1]
        if last.get("state") == "CHANGES_REQUESTED":
            deny("authorized reviewer has outstanding changes request")
        if last.get("state") == "DISMISSED":
            deny("authorized reviewer has dismissed review")
        if last["user"]["login"].casefold() == claimed_login.casefold():
            candidates.append((user_id, last_time, last))
    if len(candidates) != 1:
        deny("claim cannot identify exactly one independent stable reviewer")
    _, review_time, last = candidates[0]
    if last.get("state") != "APPROVED":
        deny("latest review by attester is no longer approved")
    if review_time != claim_time or last.get("commit_id") != head_sha:
        deny("approval does not match claimed timestamp or exact final PR head")
    return True


def self_test():
    repo = "test-org/.github"
    head = "c" * 40
    url = f"https://github.com/{repo}/pull/17"
    when = "2026-10-01T11:00:00Z"
    merged = "2026-10-01T12:00:00Z"
    proposal = {
        "repository": "test-org/example-admin-api-server.rs",
        "trust_domain": "admin-api", "database_scope": "admin",
        "review_status": "reviewed",
    }
    row = dict(proposal, review_evidence={
        "url": url, "reviewed_by": "reviewer", "reviewed_at": when,
    })
    pr = {
        "number": 17, "state": "closed", "merged": True,
        "merged_at": merged, "html_url": url,
        "user": {"id": 20, "login": "author"},
        "base": {"ref": "main", "repo": {"full_name": repo}},
        "head": {"sha": head},
    }
    reviews = [{"id": 100, "user": {"id": 30, "login": "reviewer"},
                "state": "APPROVED", "commit_id": head, "submitted_at": when}]
    origin = {"repository": repo, "commit_sha": head,
              "path": "review-proposals/example-admin-api-server.json"}
    approvals = frozenset({30})
    scenarios = []

    def probe(name, mutate=None, accepted=False):
        candidate, pr_obj, history, payload, source, allowed = copy.deepcopy(
            (row, pr, reviews, proposal, origin, approvals))
        complete = True
        if mutate:
            changes = mutate(candidate, pr_obj, history, payload, source, allowed)
            if changes is not None:
                if "allowlist" in changes:
                    allowed = changes["allowlist"]
                if "complete" in changes:
                    complete = changes["complete"]
        actual = False
        try:
            actual = verify_reviewed_row(
                candidate, pr_obj, history, payload, source, repo_full_name=repo,
                trusted_reviewer_ids=allowed, review_history_complete=complete)
        except ReviewDenied:
            pass
        if actual != accepted:
            raise AssertionError(f"{name}: " + ("unjustified acceptance" if actual else "false rejection"))
        scenarios.append(name)

    probe("valid-approved-proposal-plus-subsequent-evidence", accepted=True)
    probe("PR-not-merged", lambda r,p,h,a,s,t: p.update(merged=False))
    probe("PR-open", lambda r,p,h,a,s,t: p.update(state="open"))
    probe("wrong-target-branch", lambda r,p,h,a,s,t: p["base"].update(ref="dev"))
    probe("foreign-target-repo", lambda r,p,h,a,s,t: p["base"]["repo"].update(full_name="other/.github"))
    probe("nonmatching-proposal", lambda r,p,h,a,s,t: a.update(database_scope="app"))
    probe("proposal-has-self-referential-evidence", lambda r,p,h,a,s,t: a.update(review_evidence={}))
    probe("proposal-not-actually-reviewed", lambda r,p,h,a,s,t: a.update(review_status="inferred-needs-review"))
    probe("applicant-mutates-fields-after-approval", lambda r,p,h,a,s,t: r.update(database_scope="app"))
    probe("wrong-proposal-commit", lambda r,p,h,a,s,t: s.update(commit_sha="d"*40))
    probe("wrong-proposal-repo", lambda r,p,h,a,s,t: s.update(repository="other/.github"))
    probe("proposal-path-traversal", lambda r,p,h,a,s,t: s.update(path="review-proposals/../secret.json"))
    probe("proposal-is-live-registry", lambda r,p,h,a,s,t: s.update(path="repository-taxonomy.json"))
    probe("wrong-evidence-url", lambda r,p,h,a,s,t: r["review_evidence"].update(url=url+"-other"))
    probe("unmerged-no-timestamp", lambda r,p,h,a,s,t: p.pop("merged_at"))
    probe("review-post-merger", lambda r,p,h,a,s,t: h[0].update(submitted_at="2026-10-02T13:00:00Z"))
    probe("approval-timestamp-spoof", lambda r,p,h,a,s,t: r["review_evidence"].update(reviewed_at="2026-10-01T11:01:00Z"))
    probe("timestamp-without-timezone", lambda r,p,h,a,s,t: r["review_evidence"].update(reviewed_at="2026-10-01T11:00:00"))
    probe("unapproved-reviewer", lambda r,p,h,a,s,t: {"allowlist": frozenset({40})})
    probe("no-trusted-reviewer", lambda r,p,h,a,s,t: {"allowlist": frozenset()})
    probe("malformed-reviewer-id", lambda r,p,h,a,s,t: {"allowlist": frozenset({True})})
    probe("username-reuse-with-different-user-id", lambda r,p,h,a,s,t: h[0]["user"].update(id=31))
    probe("self-review-same-stable-id", lambda r,p,h,a,s,t: p["user"].update(id=30,login="different-name"))
    probe("login-case-normalization", lambda r,p,h,a,s,t: h[0]["user"].update(login="REVIEWER"), accepted=True)
    probe("review-later-dismissed", lambda r,p,h,a,s,t: h.append({
        "id": 101,"user":{"id":30,"login":"reviewer"},"state":"DISMISSED",
        "submitted_at":"2026-10-01T11:01:00Z","commit_id":head}))
    probe("review-later-changes-requested", lambda r,p,h,a,s,t: h.append({
        "id":101,"user":{"id":30,"login":"reviewer"},"state":"CHANGES_REQUESTED",
        "submitted_at":"2026-10-01T11:01:00Z","commit_id":head}))
    probe("same-timestamp-higher-id-dismissal", lambda r,p,h,a,s,t: h.append({
        "id":101,"user":{"id":30,"login":"reviewer"},"state":"DISMISSED",
        "submitted_at":when,"commit_id":head}))
    probe("same-timestamp-higher-id-approval", lambda r,p,h,a,s,t: (
        h[0].update(state="DISMISSED"),h.append({
            "id":101,"user":{"id":30,"login":"reviewer"},"state":"APPROVED",
            "submitted_at":when,"commit_id":head})), accepted=True)
    probe("renamed-user-later-dismissal", lambda r,p,h,a,s,t: h.append({
        "id":101,"user":{"id":30,"login":"new-login"},"state":"DISMISSED",
        "submitted_at":"2026-10-01T11:01:00Z","commit_id":head}))
    probe("renamed-user-later-changes-requested", lambda r,p,h,a,s,t: h.append({
        "id":101,"user":{"id":30,"login":"new-login"},"state":"CHANGES_REQUESTED",
        "submitted_at":"2026-10-01T11:01:00Z","commit_id":head}))
    probe("previous-login-review-still-tracked", lambda r,p,h,a,s,t: h.insert(0,{
        "id":99,"user":{"id":30,"login":"previous-login"},"state":"COMMENTED",
        "submitted_at":"2026-10-01T10:59:00Z","commit_id":head}), accepted=True)
    probe("another-authorized-reviewer-changes-requested", lambda r,p,h,a,s,t: (
        h.append({"id":101,"user":{"id":31,"login":"second-reviewer"},
                  "state":"CHANGES_REQUESTED","submitted_at":"2026-10-01T11:02:00Z",
                  "commit_id":head}),{"allowlist":frozenset({30,31})})[1])
    probe("multiple-IDs-claim-same-login", lambda r,p,h,a,s,t: (
        h.append({"id":101,"user":{"id":31,"login":"reviewer"},
                  "state":"APPROVED","submitted_at":when,"commit_id":head}),
        {"allowlist":frozenset({30,31})})[1])
    probe("duplicate-review-ids", lambda r,p,h,a,s,t: h.append(copy.deepcopy(h[0])))
    probe("stale-approved-head", lambda r,p,h,a,s,t: h[0].update(commit_id="d"*40))
    probe("unknown-completeness", lambda r,p,h,a,s,t: {"complete":False})
    probe("completeness-integer", lambda r,p,h,a,s,t: {"complete":1})
    probe("missing-approval", lambda r,p,h,a,s,t: h.clear())
    probe("candidate-not-reviewed", lambda r,p,h,a,s,t: r.update(review_status="inferred-needs-review"))
    probe("bool-pr-number", lambda r,p,h,a,s,t: p.update(number=True))
    probe("bool-author-id", lambda r,p,h,a,s,t: p["user"].update(id=True))
    probe("wrong-claimed-login", lambda r,p,h,a,s,t: r["review_evidence"].update(reviewed_by="fake"))
    probe("blank-claimed-login", lambda r,p,h,a,s,t: r["review_evidence"].update(reviewed_by=""))
    probe("missing-approval-source", lambda r,p,h,a,s,t: s.clear())
    probe("forged-merged-time-before-review", lambda r,p,h,a,s,t: p.update(merged_at="2026-10-01T10:00:00Z"))
    print(f"review attestation prototype self-tests passed: {len(scenarios)}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if not args.self_test:
        parser.error("PRODUCTION DENY: no independent GitHub API caller or protected reviewer-ID authority")
    self_test()
