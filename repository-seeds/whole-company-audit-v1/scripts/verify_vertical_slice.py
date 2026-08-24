#!/usr/bin/env python3
"""Execute the connector-to-auditor conformance flow without network or credentials."""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path


SEED_ROOT = Path(__file__).resolve().parents[1]
AUDITOR = SEED_ROOT / "canonical-company-auditor.rs" / "Cargo.toml"
CONNECTORS = SEED_ROOT / "canonical-evidence-connectors.rs" / "Cargo.toml"
FIXTURE = (
    SEED_ROOT
    / "canonical-evidence-connectors.rs"
    / "tests"
    / "fixtures"
    / "github-pages.json"
)


def run(command: list[str], input_bytes: bytes | None = None) -> bytes:
    completed = subprocess.run(
        command,
        input=input_bytes,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    if completed.returncode != 0:
        sys.stderr.buffer.write(completed.stderr)
        raise SystemExit(completed.returncode)
    return completed.stdout


def connector_batch() -> bytes:
    return run(
        [
            "cargo",
            "run",
            "--quiet",
            "--locked",
            "--manifest-path",
            str(CONNECTORS),
            "--bin",
            "connectorctl",
            "--",
            "collect-fixture",
            str(FIXTURE),
            "tenant-a",
            "acme",
            "1767225600",
        ]
    )


def auditor(command: str, input_bytes: bytes) -> bytes:
    arguments = [
        "cargo",
        "run",
        "--quiet",
        "--locked",
        "--manifest-path",
        str(AUDITOR),
        "--bin",
        "auditorctl",
        "--",
        command,
    ]
    if command == "package":
        arguments.extend(
            ["tenant-a", "organization/acme", "1767225600", "1775001600"]
        )
    return run(arguments, input_bytes)


def main() -> None:
    first_batch = connector_batch()
    second_batch = connector_batch()
    if first_batch != second_batch:
        raise SystemExit("connector output is not deterministic")
    batch = json.loads(first_batch)
    observations = batch.get("observations", [])
    if len(observations) != 3:
        raise SystemExit(f"expected three fixture observations, got {len(observations)}")
    if any("upstream_noise" in item.get("normalized", {}) for item in observations):
        raise SystemExit("redaction boundary leaked an unknown upstream field")

    first_package = auditor("package", first_batch)
    second_package = auditor("package", second_batch)
    if first_package != second_package:
        raise SystemExit("audit package output is not deterministic")
    verification = json.loads(auditor("verify", first_package))
    if verification.get("verified") is not True or verification.get("observation_count") != 3:
        raise SystemExit("auditor did not verify the expected package")
    oscal = json.loads(auditor("oscal", first_package))
    if len(oscal.get("resources", [])) != 3:
        raise SystemExit("OSCAL boundary did not project every observation")

    print(
        "PASS: hermetic connector -> auditor package -> verification -> OSCAL "
        f"({verification['package_id']})"
    )


if __name__ == "__main__":
    main()
