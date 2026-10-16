"""Offline adversarial tests for the separately protected GitHub App check runner."""
import base64
import copy
import json
import unittest
from pathlib import Path

from scripts.independent_taxonomy_gate import (
    AdmissionError, CHECK_NAME, MAX_CONTENT, WORKFLOW_BLOB,
    GitHubClient, complete_reviews, evaluate, get_file, process, repo_parts,
    strict_json, validate_evidence,
)

REPO = "canonical-cloud/.github"
HEAD = "c" * 40
OTHER = "d" * 40
SCHEMA = json.loads(Path("repository-taxonomy.schema.json").read_text())


def baseline():
    return {
        "version": 1, "organization": "canonical-cloud",
        "inventory_visibility": "public-only",
        "governance": {
            "approval_required": True, "unverified_fields_grant_no_access": True,
            "separate_admin_deployments": True,
            "public_registry_excludes_private_repositories": True,
        },
        "repositories": [{
            "repository": "canonical-cloud/.github", "category": "governance",
            "visibility": "public", "trust_domain": "shared-library",
            "source_authority": "canonical",
            "release_unit": {"kind": "none", "identifiers": [], "status": "unverified"},
            "deployment_targets": {
                "status": "unverified", "environments": [], "vpcs": [],
                "cloud_identities": [],
            },
            "database_scope": "none", "lifecycle": "unverified",
            "review_status": "inferred-needs-review",
        }],
    }


class FakeGitHub:
    def __init__(self):
        self.pr = {
            "number": 7, "state": "open",
            "head": {"sha": HEAD},
            "base": {"ref": "main", "repo": {"full_name": REPO}},
        }
        self.schema = copy.deepcopy(SCHEMA)
        self.registry = baseline()
        self.workflow_sha = WORKFLOW_BLOB
        self.deleted = set()
        self.published = []
        self.override_review_pages = {}
        self.api_extra = {}

    def _contents(self, path):
        if path in self.deleted:
            raise AdmissionError("missing required repository file")
        if path == ".github/workflows/repository-taxonomy.yml":
            payload = b"pinned workflow"
            sha = self.workflow_sha
        elif path == "repository-taxonomy.schema.json":
            payload = json.dumps(self.schema).encode()
            sha = OTHER
        elif path == "repository-taxonomy.json":
            payload = json.dumps(self.registry).encode()
            sha = OTHER
        else:
            raise AdmissionError("unexpected proposal path")
        return {
            "type": "file", "sha": sha, "size": len(payload),
            "encoding": "base64", "content": base64.b64encode(payload).decode(),
        }

    def get(self, path):
        if path == "/repos/canonical-cloud/.github/pulls/7":
            return copy.deepcopy(self.pr)
        if path in self.api_extra:
            return copy.deepcopy(self.api_extra[path])
        if path.startswith("/repos/canonical-cloud/.github/contents/"):
            tail = path.split("/contents/", 1)[1]
            local, sep, sha = tail.partition("?ref=")
            if not sep or sha != HEAD:
                raise AdmissionError("content ref not immutable PR head")
            return self._contents(local)
        if "/reviews?" in path:
            n = int(path.rsplit("&page=", 1)[1])
            return copy.deepcopy(self.override_review_pages.get(n, []))
        raise AdmissionError("unexpected GitHub API route " + path)

    def publish(self, repo, sha, conclusion, message):
        self.published.append((repo, sha, conclusion, message))
        return {"name": CHECK_NAME, "conclusion": conclusion}


class GateTests(unittest.TestCase):
    def setUp(self):
        self.gh = FakeGitHub()

    def test_baseline(self):
        self.assertEqual(evaluate(self.gh, REPO, 7, SCHEMA, frozenset()),
                         {"head_sha": HEAD, "repositories": 1, "reviewed": 0})

    def test_success_check_at_exact_head(self):
        self.assertTrue(process(self.gh, REPO, 7, SCHEMA, frozenset()))
        self.assertEqual(self.gh.published[-1][:3], (REPO, HEAD, "success"))

    def test_deleted_workflow_fails(self):
        self.gh.deleted.add(".github/workflows/repository-taxonomy.yml")
        with self.assertRaises(AdmissionError):
            evaluate(self.gh, REPO, 7, SCHEMA, frozenset())

    def test_deleted_workflow_reports_failed_check(self):
        self.gh.deleted.add(".github/workflows/repository-taxonomy.yml")
        self.assertFalse(process(self.gh, REPO, 7, SCHEMA, frozenset()))
        self.assertEqual(self.gh.published[-1][2], "failure")

    def test_modified_workflow_fails(self):
        self.gh.workflow_sha = OTHER
        with self.assertRaisesRegex(AdmissionError, "workflow"):
            evaluate(self.gh, REPO, 7, SCHEMA, frozenset())

    def test_weakened_schema_fails(self):
        self.gh.schema["properties"]["governance"]["properties"]["approval_required"]["const"] = False
        with self.assertRaisesRegex(AdmissionError, "schema"):
            evaluate(self.gh, REPO, 7, SCHEMA, frozenset())

    def test_valid_schema_but_false_governance_fails(self):
        self.gh.registry["governance"]["approval_required"] = False
        with self.assertRaisesRegex(AdmissionError, "taxonomy"):
            evaluate(self.gh, REPO, 7, SCHEMA, frozenset())

    def test_unverified_deployed_cloud_claim_fails(self):
        self.gh.registry["repositories"][0]["deployment_targets"]["vpcs"] = ["vpc-unknown"]
        with self.assertRaises(AdmissionError):
            evaluate(self.gh, REPO, 7, SCHEMA, frozenset())

    def test_reviewed_promotion_without_trusted_reviewer_ids_fails(self):
        self.gh.registry["repositories"][0]["review_status"] = "reviewed"
        self.gh.registry["repositories"][0]["review_evidence"] = {
            "url": "https://github.com/canonical-cloud/.github/pull/2",
            "reviewed_by": "fake", "reviewed_at": "2026-10-01T00:00:00Z"}
        with self.assertRaisesRegex(AdmissionError, "trusted approver"):
            evaluate(self.gh, REPO, 7, SCHEMA, frozenset())

    def test_bad_review_url_fails(self):
        self.gh.registry["repositories"][0]["review_status"] = "reviewed"
        self.gh.registry["repositories"][0]["review_evidence"] = {
            "url": "https://github.com/other/.github/pull/3",
            "reviewed_by": "fake", "reviewed_at": "2026-10-01T00:00:00Z"}
        with self.assertRaisesRegex(AdmissionError, "reference"):
            evaluate(self.gh, REPO, 7, SCHEMA, frozenset({123}))

    def test_foreign_repository_disallowed(self):
        with self.assertRaises(AdmissionError):
            repo_parts("other/.github")

    def test_app_repository_not_other_source_repo(self):
        with self.assertRaises(AdmissionError):
            repo_parts("canonical-cloud/app")

    def test_non_main_target_fails(self):
        self.gh.pr["base"]["ref"] = "feature"
        with self.assertRaisesRegex(AdmissionError, "main"):
            evaluate(self.gh, REPO, 7, SCHEMA, frozenset())

    def test_other_target_repository_fails(self):
        self.gh.pr["base"]["repo"]["full_name"] = "shared-auth/.github"
        with self.assertRaises(AdmissionError):
            evaluate(self.gh, REPO, 7, SCHEMA, frozenset())

    def test_invalid_head_ref_fails(self):
        self.gh.pr["head"]["sha"] = "main"
        with self.assertRaises(AdmissionError):
            evaluate(self.gh, REPO, 7, SCHEMA, frozenset())

    def test_workflow_checks_exact_head_sha(self):
        # FakeGitHub refuses any request that uses a branch ref instead of HEAD.
        self.assertEqual(evaluate(self.gh, REPO, 7, SCHEMA, frozenset())["head_sha"], HEAD)

    def test_duplicate_json_key_fails(self):
        with self.assertRaisesRegex(AdmissionError, "duplicate"):
            strict_json(b'{"a":1,"a":2}')

    def test_nonfinite_json_fails(self):
        with self.assertRaises(AdmissionError):
            strict_json(b'{"v":NaN}')

    def test_oversized_json_fails(self):
        with self.assertRaises(AdmissionError):
            strict_json(b" " * (MAX_CONTENT + 1))

    def test_malformed_utf8_fails(self):
        with self.assertRaises(AdmissionError):
            strict_json(b"\xff")

    def test_complete_review_one_page(self):
        self.gh.override_review_pages = {1: [{"id": 1}]}
        self.assertEqual(len(complete_reviews(self.gh, REPO, 2)), 1)

    def test_complete_review_exact_hundred_requires_next_page(self):
        self.gh.override_review_pages = {1: [{"id": i} for i in range(100)],
                                         2: [{"id": 100}]}
        self.assertEqual(len(complete_reviews(self.gh, REPO, 2)), 101)

    def test_review_page_wrong_type_fails(self):
        self.gh.override_review_pages = {1: {"id": 1}}
        with self.assertRaises(AdmissionError):
            complete_reviews(self.gh, REPO, 2)

    def test_pagination_failure_is_not_silently_complete(self):
        self.gh.override_review_pages = {1: [{"id": i} for i in range(100)]}
        old = self.gh.get
        def broken(path):
            if "page=2" in path:
                raise AdmissionError("GitHub API failed")
            return old(path)
        self.gh.get = broken
        with self.assertRaises(AdmissionError):
            complete_reviews(self.gh, REPO, 2)

    def test_sha_of_content_checked(self):
        self.gh.workflow_sha = "not a git sha"
        with self.assertRaises(AdmissionError):
            get_file(self.gh, REPO, ".github/workflows/repository-taxonomy.yml", HEAD)

    def test_check_run_when_schema_downgraded_fails(self):
        self.gh.schema["properties"]["governance"]["properties"]["approval_required"]["const"] = False
        self.assertFalse(process(self.gh, REPO, 7, SCHEMA, frozenset()))
        self.assertEqual(self.gh.published[-1][2], "failure")

    def test_token_is_required_to_use_real_http_client(self):
        with self.assertRaises(AdmissionError):
            GitHubClient("")

    def test_bool_pr_number_fails(self):
        with self.assertRaises(AdmissionError):
            evaluate(self.gh, REPO, True, SCHEMA, frozenset())

    def test_review_attestation_requires_authenticated_evidence(self):
        row = {"repository": "canonical-cloud/thing", "review_status": "reviewed",
               "review_evidence": {"url": "https://github.com/canonical-cloud/.github/pull/2"}}
        with self.assertRaises(AdmissionError):
            validate_evidence(self.gh, REPO, row, frozenset({123}))


if __name__ == "__main__":
    unittest.main()
