#!/usr/bin/env python3
"""Independent GitHub App taxonomy check runner — DEPLOYMENT DISABLED BY DEFAULT.

This is a *staging* implementation: its source is still on a PR branch. DO NOT
give that branch tokens. A security administrator must first promote both this
code and its imports to a separately protected verifier release, install an
independent GitHub App and require THAT App's check on protected branches.

Never run untrusted repository scripts, checkout commands or dependencies.
"""
import argparse
import base64
import copy
import hashlib
import json
import os
import re
import sys
import urllib.error
import urllib.parse
import urllib.request

from scripts.validate_repository_taxonomy import check as validate_taxonomy, PolicyError
from scripts.verify_taxonomy_review import verify_reviewed_row, ReviewDenied

CHECK_NAME = "taxonomy-independent-gate"
WORKFLOW_BLOB = "9f8aed7672bf9e5021a61f8424268a550f738000"
SCHEMA_BLOB = "d2ec393f368d17793c6812b4867d3fedb50ef578"
PRIVATE_ORGS = frozenset({"beamscale", "ores-data-platform"})
# Hard stop: evidence-backed promotions require separate security-reviewed deployment.
ALLOW_REVIEWED_PROMOTIONS = False
ALLOWED_ORGS = frozenset({
    "canonical-cloud", "fiducia-cloud", "shared-auth", "beamscale",
    "ores-data-platform", "ores-chat", "3FA-app", "messaging-intel",
    "agent-pontifex", "gha-indie-worker",
})
MAX_CONTENT = 2_000_000
MAX_REVIEWS = 2_000
VALID_SHA = re.compile(r"[0-9a-f]{40}\Z")
SAFE_OWNER = re.compile(r"[A-Za-z0-9_.-]+\Z")


class AdmissionError(ValueError):
    pass


def reject(reason):
    raise AdmissionError(reason)


def pairs_unique(pairs):
    result = {}
    for key, val in pairs:
        if key in result:
            reject(f"duplicate JSON key: {key}")
        result[key] = val
    return result


def strict_json(raw):
    if len(raw) > MAX_CONTENT:
        reject("JSON payload exceeds configured maximum")
    def no_constant(v):
        reject(f"non-JSON constant {v}")
    try:
        return json.loads(raw.decode("utf-8"), object_pairs_hook=pairs_unique,
                          parse_constant=no_constant)
    except (UnicodeDecodeError, json.JSONDecodeError) as exc:
        reject(f"malformed UTF-8 JSON: {exc}")


def require_sha(value):
    if not isinstance(value, str) or not VALID_SHA.fullmatch(value):
        reject("missing or malformed immutable head SHA")
    return value


class NoRedirect(urllib.request.HTTPRedirectHandler):
    def redirect_request(self, req, fp, code, msg, headers, newurl):
        reject("unexpected GitHub API redirect")


class GitHubClient:
    """Only talks to fixed api.github.com REST endpoints, no redirects."""
    def __init__(self, token):
        if not isinstance(token, str) or not token or "\n" in token or "\r" in token:
            reject("GitHub App installation token not provisioned")
        self.token = token
        self.opener = urllib.request.build_opener(NoRedirect)

    def request(self, method, path, body=None):
        if not path.startswith("/") or "//" in path or ".." in path:
            reject("unsafe API path")
        url = "https://api.github.com" + path
        data = json.dumps(body).encode("utf-8") if body is not None else None
        req = urllib.request.Request(
            url, data=data, method=method,
            headers={
                "Accept": "application/vnd.github+json",
                "X-GitHub-Api-Version": "2026-03-10",
                "Authorization": "Bearer " + self.token,
                "User-Agent": "taxonomy-independent-gate-staging",
                "Content-Type": "application/json",
            },
        )
        try:
            with self.opener.open(req, timeout=20) as stream:
                raw = stream.read(MAX_CONTENT + 1)
        except (urllib.error.URLError, TimeoutError) as exc:
            reject(f"GitHub API unavailable for {method} {path}: {type(exc).__name__}")
        return strict_json(raw)

    def get(self, path):
        return self.request("GET", path)

    def publish(self, repo, sha, conclusion, message):
        owner, name = repo.split("/", 1)
        # The App must have Checks:write; require this named check from this exact
        # GitHub App in administrator-controlled rules, not simply a check name.
        return self.request(
            "POST", f"/repos/{owner}/{name}/check-runs",
            {"name": CHECK_NAME, "head_sha": require_sha(sha),
             "status": "completed", "conclusion": conclusion,
             "output": {"title": "Independent taxonomy admission",
                        "summary": message[:500]}},
        )


def repo_parts(repo):
    if not isinstance(repo, str) or repo.count("/") != 1:
        reject("invalid protected repo")
    owner, name = repo.split("/")
    if owner not in ALLOWED_ORGS or name != ".github":
        reject("repository not on protected pilot allowlist")
    if not SAFE_OWNER.fullmatch(owner):
        reject("unsafe organization")
    return owner, name


def get_file(client, repo, path, ref):
    owner, name = repo_parts(repo)
    if not isinstance(path, str) or path.startswith("/") or ".." in path:
        reject("unsafe taxonomy content path")
    ref = require_sha(ref)
    path_encoded = "/".join(urllib.parse.quote(p, safe="") for p in path.split("/"))
    result = client.get(
        f"/repos/{owner}/{name}/contents/{path_encoded}?ref={ref}")
    if not isinstance(result, dict) or result.get("type") != "file":
        reject(f"{path}: not a GitHub file")
    if not isinstance(result.get("sha"), str) or not VALID_SHA.fullmatch(result["sha"]):
        reject(f"{path}: missing source blob")
    if result.get("size", 0) > MAX_CONTENT or result.get("encoding") != "base64":
        reject(f"{path}: unsupported or oversized file")
    try:
        raw = base64.b64decode(result["content"].replace("\n", ""), validate=True)
    except (ValueError, TypeError) as exc:
        reject(f"{path}: invalid base64: {type(exc).__name__}")
    if len(raw) > MAX_CONTENT:
        reject(f"{path}: oversize after decoding")
    return raw, result["sha"]


def read_registry(client, repo, sha, path):
    blob, _ = get_file(client, repo, path, sha)
    return strict_json(blob)


def complete_reviews(client, repo, n):
    owner, name = repo_parts(repo)
    if type(n) is not int or n < 1:
        reject("invalid reviewed PR number")
    reviews = []
    # Read every page; a partial review history is NEVER treated as complete.
    for page in range(1, MAX_REVIEWS // 100 + 2):
        batch = client.get(
            f"/repos/{owner}/{name}/pulls/{n}/reviews?per_page=100&page={page}")
        if not isinstance(batch, list) or len(batch) > 100:
            reject("invalid review page")
        reviews.extend(batch)
        if len(reviews) > MAX_REVIEWS:
            reject("review history exceeds bounded complete scan")
        if len(batch) < 100:
            return reviews
    reject("review pagination incomplete")


def validate_evidence(client, repo, row, reviewer_ids):
    evidence = row.get("review_evidence")
    if not isinstance(evidence, dict):
        reject("reviewed record has no review evidence")
    owner, name = repo_parts(repo)
    url = evidence.get("url")
    if not isinstance(url, str):
        reject("invalid review reference")
    prefix = f"https://github.com/{repo}/pull/"
    suffix = url[len(prefix):] if url.startswith(prefix) else ""
    if not suffix.isdecimal() or suffix.startswith("0"):
        reject("review PR reference is not a valid governance PR")
    number = int(suffix)
    if number > 2**31:
        reject("review PR number out of bounds")
    pr = client.get(f"/repos/{owner}/{name}/pulls/{number}")
    if not isinstance(pr, dict):
        reject("missing trusted PR evidence")
    head = pr.get("head")
    head_sha = require_sha(head.get("sha") if isinstance(head, dict) else None)
    slug = row.get("repository", "").split("/")[-1]
    if not SAFE_OWNER.fullmatch(slug):
        reject("invalid proposal repository slug")
    path = f"review-proposals/{slug}.json"
    proposal = read_registry(client, repo, head_sha, path)
    history = complete_reviews(client, repo, number)
    try:
        verify_reviewed_row(
            row, pr, history, proposal,
            {"repository": repo, "commit_sha": head_sha, "path": path},
            repo_full_name=repo, trusted_reviewer_ids=reviewer_ids,
            review_history_complete=True)
    except (ReviewDenied, ValueError, KeyError, TypeError) as exc:
        reject(f"review provenance rejected: {exc}")


def evaluate(client, repo, pr_number, trusted_schema, reviewer_ids, *,
             pinned_workflow_blob=WORKFLOW_BLOB):
    """Validate proposed PR head *as data*; never execute candidate code.

    All trusted args and the imported validation functions MUST be located in
    a separately protected verifier deployment, not in the PR being tested.
    """
    owner, name = repo_parts(repo)
    if type(pr_number) is not int or pr_number < 1:
        reject("invalid PR number")
    pr = client.get(f"/repos/{owner}/{name}/pulls/{pr_number}")
    if not isinstance(pr, dict) or not isinstance(pr.get("head"), dict):
        reject("missing latest PR metadata")
    head = require_sha(pr["head"].get("sha"))
    metadata = client.get(f"/repos/{owner}/{name}")
    if (not isinstance(metadata, dict) or type(metadata.get("private")) is not bool
            or metadata["private"] != (owner in PRIVATE_ORGS)):
        reject("repository visibility no longer matches approved inventory")
    if (not isinstance(pr.get("base"), dict) or pr["base"].get("ref") != "main"
            or not isinstance(pr["base"].get("repo"), dict)
            or pr["base"]["repo"].get("full_name") != repo):
        reject("target is not protected main")
    _, workflow_sha = get_file(client, repo, ".github/workflows/repository-taxonomy.yml", head)
    if workflow_sha != pinned_workflow_blob:
        reject("PR deletes or changes the required governance workflow")
    local_schema = read_registry(client, repo, head, "repository-taxonomy.schema.json")
    expected = copy.deepcopy(trusted_schema)
    expected["properties"]["organization"] = {"const": owner}
    if local_schema != expected:
        reject("PR schema differs from independently pinned authority")
    registry = read_registry(client, repo, head, "repository-taxonomy.json")
    if not isinstance(registry, dict) or not isinstance(registry.get("repositories"), list):
        reject("invalid repository registry")
    reviewed = [row for row in registry["repositories"]
                if isinstance(row, dict) and row.get("review_status") == "reviewed"]
    if reviewed and not ALLOW_REVIEWED_PROMOTIONS:
        reject("reviewed promotions remain disabled pending independent infrastructure attestation")
    if reviewed and not reviewer_ids:
        reject("reviewed promotions disabled: trusted approver IDs unavailable")
    for row in reviewed:
        validate_evidence(client, repo, row, reviewer_ids)
    # Fixture override is safe ONLY now: authenticated independent attestations
    # were checked for EVERY reviewed row by this protected evaluator.
    try:
        count = validate_taxonomy(
            registry, expected, owner, public_repo=(owner not in PRIVATE_ORGS),
            _fixture_review_authorization=True)
    except (PolicyError, ValueError, TypeError, KeyError) as exc:
        reject(f"taxonomy policy rejected: {exc}")
    return {"head_sha": head, "repositories": count, "reviewed": len(reviewed)}


def process(client, repo, pr_number, trusted_schema, reviewer_ids):
    """Report a completed Check Run for the CURRENT PR SHA if it can be fetched."""
    owner, name = repo_parts(repo)
    pr = client.get(f"/repos/{owner}/{name}/pulls/{pr_number}")
    if not isinstance(pr, dict) or not isinstance(pr.get("head"), dict):
        reject("cannot identify PR SHA for check run")
    head = require_sha(pr["head"].get("sha"))
    try:
        result = evaluate(client, repo, pr_number, trusted_schema, reviewer_ids)
        if result["head_sha"] != head:
            reject("PR changed while evaluation ran; retry latest head")
        client.publish(repo, head, "success",
                       f"PASS: {result['repositories']} repositories; "
                       f"{result['reviewed']} approved promotions.")
        return True
    except (AdmissionError, ReviewDenied, PolicyError) as error:
        client.publish(repo, head, "failure", "FAIL: " + str(error))
        return False


def main():
    parser = argparse.ArgumentParser(description="Protected GitHub App check staging runner")
    parser.add_argument("--evaluate", nargs=2, metavar=("ORG_GITHUB_REPO", "PR_NUMBER"))
    args = parser.parse_args()
    if not args.evaluate:
        parser.error("production requires --evaluate and independently administered installation")
    if os.environ.get("INDEPENDENT_TAXONOMY_GATE_ENABLED") != "yes":
        parser.error("disabled until independently protected GitHub App is installed")
    repo, n = args.evaluate
    try:
        n = int(n)
        reviewer_text = os.environ.get("GATE_TRUSTED_REVIEWER_IDS", "")
        reviewer_ids = frozenset(int(x) for x in reviewer_text.split(",") if x)
        token = os.environ.get("GITHUB_INSTALLATION_TOKEN", "")
        schema_file = os.environ.get("GATE_TRUSTED_SCHEMA_PATH", "")
        if not schema_file:
            reject("trusted schema source missing")
        from pathlib import Path
        trusted_path = Path(schema_file)
        if not trusted_path.is_absolute() or trusted_path.is_symlink() or not trusted_path.is_file():
            reject("trusted schema must be a regular, administrator-owned absolute file")
        schema_bytes = trusted_path.read_bytes()
        blob = hashlib.sha1(b"blob " + str(len(schema_bytes)).encode("ascii") + b"\0" + schema_bytes).hexdigest()
        if blob != SCHEMA_BLOB:
            reject("trusted schema does not match independently pinned blob")
        schema = strict_json(schema_bytes)
        client = GitHubClient(token)
        successful = process(client, repo, n, schema, reviewer_ids)
        sys.exit(0 if successful else 1)
    except (AdmissionError, OSError, ValueError) as exc:
        print(f"independent taxonomy gate FAILED CLOSED: {exc}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
