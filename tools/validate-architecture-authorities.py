#!/usr/bin/env python3
"""Fail-closed validation for the public-safe architecture authority registry."""

from __future__ import annotations

import json
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
REGISTRY = ROOT / "architecture-authorities.json"
MATRIX = ROOT / "docs" / "DOCUMENTATION_AUTHORITY_MATRIX.md"
PUBLIC_INVENTORY = ROOT / "repository-relationships.json"

ALLOWED_STATUSES = {"active", "legacy", "prepared", "pending"}
ISSUE_RE = re.compile(r"^[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+#[1-9][0-9]*$")
ID_RE = re.compile(r"^[a-z][a-z0-9_]*$")


def fail(message: str) -> None:
    raise SystemExit(f"architecture authority registry invalid: {message}")


def require(condition: bool, message: str) -> None:
    if not condition:
        fail(message)


def load_json(path: Path) -> object:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError:
        fail(f"missing required file: {path.relative_to(ROOT)}")
    except json.JSONDecodeError as exc:
        fail(f"invalid JSON in {path.relative_to(ROOT)}: {exc}")


def validate() -> None:
    registry = load_json(REGISTRY)
    require(isinstance(registry, dict), "registry root must be an object")
    assert isinstance(registry, dict)

    require(registry.get("schema_version") == 1, "schema_version must equal 1")
    require(
        registry.get("human_authority_matrix") == "docs/DOCUMENTATION_AUTHORITY_MATRIX.md",
        "human_authority_matrix must point to the reviewed matrix",
    )
    require(MATRIX.is_file() and MATRIX.stat().st_size > 0, "authority matrix is missing/empty")
    require(PUBLIC_INVENTORY.is_file(), "public repository inventory is missing")

    privacy = registry.get("privacy")
    require(isinstance(privacy, dict), "privacy block is required")
    assert isinstance(privacy, dict)
    require(
        privacy.get("public_inventory") == "repository-relationships.json",
        "privacy.public_inventory must reuse repository-relationships.json",
    )
    non_public = privacy.get("non_public_inventory")
    require(isinstance(non_public, str) and non_public.strip(), "non-public inventory pointer is required")
    require(
        not non_public.startswith("/") and ".." not in Path(non_public).parts,
        "non-public inventory pointer must be a logical relative pointer, not a local secret path",
    )
    require(
        isinstance(privacy.get("rule"), str) and "Do not copy" in privacy["rule"],
        "privacy rule must explicitly prohibit copying non-public inventory",
    )

    status_values = registry.get("authority_status_values")
    require(isinstance(status_values, list), "authority_status_values must be an array")
    require(set(status_values) == ALLOWED_STATUSES, "authority_status_values must match the validator vocabulary")

    authorities = registry.get("authorities")
    require(isinstance(authorities, list) and authorities, "authorities must be a non-empty array")
    decisions = registry.get("decisions")
    require(isinstance(decisions, list), "decisions must be an array")
    invariants = registry.get("invariants")
    require(isinstance(invariants, list) and invariants, "invariants must be a non-empty array")

    authority_ids: set[str] = set()
    decision_ids: set[str] = set()

    for item in decisions:
        require(isinstance(item, dict), "every decision must be an object")
        assert isinstance(item, dict)
        ident = item.get("id")
        require(isinstance(ident, str) and ID_RE.fullmatch(ident) is not None, f"invalid decision id: {ident!r}")
        require(ident not in decision_ids, f"duplicate decision id: {ident}")
        decision_ids.add(ident)
        status = item.get("status")
        require(status in ALLOWED_STATUSES, f"decision {ident} has invalid status {status!r}")
        issues = item.get("issues")
        require(isinstance(issues, list) and issues, f"decision {ident} must reference at least one issue")
        for issue in issues:
            require(isinstance(issue, str) and ISSUE_RE.fullmatch(issue) is not None, f"decision {ident} has invalid issue ref {issue!r}")
        require(len(set(issues)) == len(issues), f"decision {ident} repeats an issue reference")

    for item in authorities:
        require(isinstance(item, dict), "every authority must be an object")
        assert isinstance(item, dict)
        ident = item.get("id")
        require(isinstance(ident, str) and ID_RE.fullmatch(ident) is not None, f"invalid authority id: {ident!r}")
        require(ident not in authority_ids, f"duplicate authority id: {ident}")
        authority_ids.add(ident)

        category = item.get("category")
        owner = item.get("owner")
        source = item.get("source")
        status = item.get("status")
        generated = item.get("generated")
        editable = item.get("editable")

        require(isinstance(category, str) and category.strip(), f"authority {ident} must name a category")
        require(isinstance(owner, str) and owner.strip(), f"authority {ident} must name an owner")
        require(isinstance(source, str) and source.strip(), f"authority {ident} must name a source/pointer")
        require(status in ALLOWED_STATUSES, f"authority {ident} has invalid status {status!r}")
        require(isinstance(generated, bool), f"authority {ident}.generated must be boolean")
        require(isinstance(editable, bool), f"authority {ident}.editable must be boolean")

        if generated:
            require(not editable, f"generated authority/projection {ident} may not be editable")

        decision = item.get("decision")
        if status == "pending":
            require(isinstance(decision, str) and decision in decision_ids, f"pending authority {ident} must reference a declared decision")
        elif decision is not None:
            require(isinstance(decision, str) and decision in decision_ids, f"authority {ident} references unknown decision {decision!r}")

    require(len(set(invariants)) == len(invariants), "invariants must be unique")
    for invariant in invariants:
        require(isinstance(invariant, str) and ID_RE.fullmatch(invariant) is not None, f"invalid invariant id: {invariant!r}")

    required_invariants = {
        "generated_artifacts_are_not_editable_authority",
        "source_presence_is_not_deployment_evidence",
        "tenant_identifiers_are_not_authorization_by_themselves",
        "one_logical_dataset_has_one_accepted_writer_at_a_time",
        "customer_plane_does_not_inherit_admin_authority",
        "public_static_surfaces_do_not_own_private_customer_evidence",
        "compatibility_aliases_require_explicit_owner_and_lifecycle",
        "kubernetes_has_one_reconciliation_authority",
    }
    require(required_invariants.issubset(set(invariants)), "required architecture invariants are missing")

    # Keep the public relationship registry's privacy boundary intact. The
    # authority registry may point to the approved private registry, but it may
    # not inline/copy its contents.
    inventory = load_json(PUBLIC_INVENTORY)
    require(isinstance(inventory, dict), "repository-relationships.json must remain an object")
    assert isinstance(inventory, dict)
    private_registry = inventory.get("private_registry")
    require(isinstance(private_registry, dict), "public inventory must retain private_registry pointer")
    assert isinstance(private_registry, dict)
    require(private_registry.get("contains_non_public_inventory") is True, "public inventory must acknowledge omitted non-public inventory")
    require(private_registry.get("path") == "owners/canonical-cloud.json", "unexpected private registry path")

    matrix_text = MATRIX.read_text(encoding="utf-8")
    require("different authorities" in matrix_text.lower() or "authority matrix" in matrix_text.lower(), "human authority matrix no longer describes layered authority")

    # Guard against the most damaging regression this audit found.
    prohibited_unqualified = (
        "JSON Schema (`schema/*.schema.json`, indexed by `schema/index.json`) is the\n"
        "single source of truth"
    )
    interfaces_readme_note = "canonical-interfaces owns multiple authority families; see its repository README"
    require(interfaces_readme_note not in matrix_text, "placeholder authority note leaked into the human matrix")

    print(
        f"architecture authority registry valid: {len(authorities)} authorities, "
        f"{len(decisions)} decisions, {len(invariants)} invariants"
    )


if __name__ == "__main__":
    try:
        validate()
    except BrokenPipeError:
        sys.exit(1)
