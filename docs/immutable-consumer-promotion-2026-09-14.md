# Immutable consumer promotion

Downstream consumers may advance only from immutable, reproducible release evidence.

- Dependency and interface bumps must identify the exact source commit, immutable release tag or package version, and artifact/package digest when one exists.
- Floating branches such as `main`, mutable tags, or an unverified version string are not promotion evidence.
- Zed-managed dependencies must preserve the resolved lock/provenance record and pass clean registry readback plus deterministic `--frozen` replay before promotion.
- Authored TypeSpec and authored JSON Schema Draft 2020-12 remain independent peer authorities; generated projections do not replace TJSV fail-closed parity evidence.
- CI and release automation must test the same immutable source identity that is proposed for consumption. A green run on a later branch head cannot validate an earlier or different artifact.
- Security, parity, reproducibility, provenance, and required integration/system-test failures block the dependency advance instead of being converted to advisory success.
- Promotion records may contain identifiers, hashes, types, and requiredness, but must never embed secret values.

This keeps dependency upgrades auditable and prevents mutable repository state from silently changing what a consumer receives.
