#!/usr/bin/env python3
"""Pure review-admission predicate for future independently governed use.

SECURITY: This module deliberately has no 'accept this local JSON' CLI mode.
A protected caller must retrieve PRs, reviews, and the approved registry blob
from GitHub's authenticated API using a trusted installation identity. A
candidate PR may supply only its request data, never the trusted approver set.
The existing taxonomy CI keeps reviewed promotions disabled.
"""
import argparse
import copy
import datetime
import json
import re


class ReviewDenied(ValueError):
    pass


SHA = re.compile(r"[0-9a-f]{40}\Z")
LOGIN = re.compile(r"[A-Za-z0-9-]+\Z")


def deny(message):
    raise ReviewDenied(message)


def timestamp(value):
    if not isinstance(value, str):
        deny("review timestamp is not a string")
    try:
        parsed = datetime.datetime.fromisoformat(value.replace("Z", "+00:00"))
    except ValueError:
        deny("malformed review timestamp")
    if parsed.tzinfo is None or parsed.utcoffset() is None:
        deny("review timestamp lacks timezone")
    return parsed.astimezone(datetime.timezone.utc)


def verify_reviewed_row(row, pr, reviews, approved_row, *, repo_full_name, trusted_reviewers):
    """Require a reviewed and *merged* PR for exactly the approved row bytes.

    Trusted inputs: pr, reviews, and approved_row must be retrieved from
    authenticated GitHub APIs, approved_row at pr.head.sha; repo_full_name and
    trusted_reviewers must come from independently protected policy configuration.
    Passing PR-authored substitutes for these inputs is unsafe.
    """
    if not isinstance(trusted_reviewers, (frozenset, set)) or not trusted_reviewers:
        deny("trusted approver allowlist unavailable")
    if any(not isinstance(login, str) or not LOGIN.fullmatch(login) for login in trusted_reviewers):
        deny("invalid trusted approver")
    if not isinstance(row, dict) or row.get("review_status") != "reviewed":
        deny("not a reviewed classification")
    if not isinstance(approved_row, dict) or row != approved_row:
        deny("record differs from approved head commit")
    if not isinstance(repo_full_name, str) or not re.fullmatch(
            r"[A-Za-z0-9_.-]+/\.github", repo_full_name):
        deny("invalid expected governance repository")
    if not isinstance(pr, dict) or not isinstance(reviews, list):
        deny("GitHub evidence missing")
    evidence = row.get("review_evidence")
    if not isinstance(evidence, dict):
        deny("review evidence missing")
    username = evidence.get("reviewed_by")
    if not isinstance(username, str) or username not in trusted_reviewers:
        deny("reviewer is not independently authorized")
    if pr.get("merged") is not True or pr.get("state") != "closed":
        deny("evidence PR is not merged")
    base = pr.get("base")
    if not isinstance(base, dict) or base.get("ref") != "main":
        deny("review was not merged into protected main")
    if not isinstance(base.get("repo"), dict) or base["repo"].get("full_name") != repo_full_name:
        deny("evidence PR belongs to different governance repository")
    head = pr.get("head")
    if not isinstance(head, dict):
        deny("missing PR head")
    head_sha = head.get("sha")
    if not isinstance(head_sha, str) or not SHA.fullmatch(head_sha):
        deny("invalid PR head sha")
    if not isinstance(pr.get("user"), dict):
        deny("missing PR author")
    author = pr["user"].get("login")
    if isinstance(author, str) and author.casefold() == username.casefold():
        deny("author cannot attest their own change")
    if not isinstance(pr.get("number"), int) or isinstance(pr["number"], bool) or pr["number"] < 1:
        deny("invalid evidence PR number")
    canonical_url = f"https://github.com/{repo_full_name}/pull/{pr['number']}"
    if evidence.get("url") != canonical_url or pr.get("html_url") != canonical_url:
        deny("evidence URL does not identify this exact governance PR")
    required_time = timestamp(evidence.get("reviewed_at"))
    history = []
    for review in reviews:
        if not isinstance(review, dict) or not isinstance(review.get("user"), dict):
            deny("malformed review history")
        reviewer = review["user"].get("login")
        if not isinstance(reviewer, str):
            deny("malformed review author")
        if reviewer.casefold() == username.casefold():
            history.append((timestamp(review.get("submitted_at")), review))
    if not history:
        deny("independent reviewer did not review PR")
    history.sort(key=lambda x: x[0])
    last_time, last_review = history[-1]
    if last_review.get("state") != "APPROVED":
        deny("latest authorized review is not approved")
    if last_time != required_time:
        deny("claimed review timestamp mismatches GitHub")
    if last_review.get("commit_id") != head_sha:
        deny("approval was not granted for final PR head")
    return True


def self_test():
    sha = "c" * 40
    repo = "test-org/.github"
    url = f"https://github.com/{repo}/pull/17"
    date = "2026-10-01T11:00:00Z"
    row = {
        "repository": "test-org/example-admin-api-server.rs",
        "trust_domain": "admin-api",
        "database_scope": "admin",
        "review_status": "reviewed",
        "review_evidence": {"url": url, "reviewed_by": "reviewer", "reviewed_at": date},
    }
    pr = {
        "number": 17, "state": "closed", "merged": True, "html_url": url,
        "user": {"login": "author"},
        "base": {"ref": "main", "repo": {"full_name": repo}},
        "head": {"sha": sha},
    }
    reviews = [{
        "user": {"login": "reviewer"}, "state": "APPROVED",
        "commit_id": sha, "submitted_at": date,
    }]
    approved_row = copy.deepcopy(row)
    approved = frozenset({"reviewer"})
    count = 0

    def probe(name, mutate=None, allowed=False):
        nonlocal count
        obj = copy.deepcopy((row, pr, reviews, approved_row, approved))
        entry, proposal, hist, approved_entry, reviewers = obj
        if mutate:
            altered = mutate(entry, proposal, hist, approved_entry, reviewers)
            if altered is not None:
                reviewers = altered
        accepted = False
        try:
            accepted = verify_reviewed_row(
                entry, proposal, hist, approved_entry, repo_full_name=repo,
                trusted_reviewers=reviewers)
        except ReviewDenied:
            accepted = False
        if accepted != allowed:
            raise AssertionError(f"{name}: {'unauthorized acceptance' if accepted else 'false rejection'}")
        count += 1

    probe("approved-matching-merged-pr", allowed=True)
    probe("unmerged-pr", lambda r, p, h, a, t: p.update(merged=False))
    probe("wrong-target-branch", lambda r, p, h, a, t: p["base"].update(ref="dev"))
    probe("cross-repository-pr", lambda r, p, h, a, t: p["base"]["repo"].update(full_name="other/.github"))
    probe("author-self-review", lambda r, p, h, a, t: p["user"].update(login="reviewer"))
    probe("untrusted-reviewer", lambda r, p, h, a, t: frozenset({"someone-else"}))
    probe("empty-trusted-allowlist", lambda r, p, h, a, t: frozenset())
    probe("stale-head-review", lambda r, p, h, a, t: h[0].update(commit_id="d" * 40))
    probe("dismissed-review", lambda r, p, h, a, t: h[0].update(state="DISMISSED"))
    probe("newer-changes-requested", lambda r, p, h, a, t: h.append({
        "user": {"login": "reviewer"}, "state": "CHANGES_REQUESTED",
        "commit_id": sha, "submitted_at": "2026-10-01T12:00:00Z"}))
    probe("record-mutated-after-approval", lambda r, p, h, a, t: r.update(database_scope="app"))
    probe("forged-review-timestamp", lambda r, p, h, a, t: r["review_evidence"].update(reviewed_at="2026-10-01T12:00:00Z"))
    probe("forged-url", lambda r, p, h, a, t: r["review_evidence"].update(url="https://github.com/test-org/.github/pull/18"))
    probe("missing-approval", lambda r, p, h, a, t: h.clear())
    probe("reviewed-state-not-set", lambda r, p, h, a, t: r.update(review_status="inferred-needs-review"))
    probe("review-without-timezone", lambda r, p, h, a, t: r["review_evidence"].update(reviewed_at="2026-10-01T11:00:00"))
    probe("malformed-pr-sha", lambda r, p, h, a, t: p["head"].update(sha="refs/heads/main"))
    probe("same-reviewer-name-case", lambda r, p, h, a, t: p["user"].update(login="REVIEWER"))
    print(f"review attestation prototype self-tests passed: {count}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if not args.self_test:
        parser.error("production admission is DISABLED: protected API fetch and reviewer authorization required")
    self_test()
