#!/usr/bin/env bash
set -euo pipefail

seed_root="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
auditor="$seed_root/canonical-company-auditor.rs"
connectors="$seed_root/canonical-evidence-connectors.rs"

python3 -m json.tool "$auditor/schemas/evidence-observation-input-v1.schema.json" >/dev/null
python3 -m json.tool "$auditor/schemas/audit-package-v1.schema.json" >/dev/null
python3 -m json.tool "$auditor/tests/fixtures/manual-observation.json" >/dev/null
python3 -m json.tool "$connectors/tests/fixtures/github-pages.json" >/dev/null

for crate in "$auditor" "$connectors"; do
  cargo fmt --manifest-path "$crate/Cargo.toml" -- --check
  cargo clippy --manifest-path "$crate/Cargo.toml" --locked --all-targets --all-features -- -D warnings
  cargo test --manifest-path "$crate/Cargo.toml" --locked --all-targets --all-features
  cargo audit --file "$crate/Cargo.lock" --deny warnings
done

python3 "$seed_root/scripts/verify_vertical_slice.py"
