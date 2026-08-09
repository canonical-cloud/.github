#!/usr/bin/env bash
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
out_dir="${1:-$script_dir/reconstructed}"
base64_stream="$out_dir/canonical-docs-ready.zip.b64"
archive="$out_dir/canonical-docs-ready.zip"
extract_dir="$out_dir/extracted"

expected_stream_sha256="3d400c0871dcd3d21f4fb588b94be1f8a8b8cd813f142aff293f74de4963f951"
expected_archive_sha256="6afa4bef55c3b69b22cc1cad0468d156bce7e7413f3af47addbcca4b25c811c4"
expected_entries=36
expected_uncompressed_bytes=103011

command -v python3 >/dev/null 2>&1 || {
  echo "Python 3 is required for portable SHA-256 and ZIP validation" >&2
  exit 127
}
command -v base64 >/dev/null 2>&1 || {
  echo "base64 is required" >&2
  exit 127
}

mkdir -p "$out_dir"
rm -f "$base64_stream" "$archive"
rm -rf "$extract_dir"

cat \
  "$script_dir/canonical-docs-ready.part00.b64" \
  "$script_dir/canonical-docs-ready.part01.b64" \
  "$script_dir/canonical-docs-ready.part02.b64" \
  "$script_dir/canonical-docs-ready.part03.b64" \
  "$script_dir/canonical-docs-ready.part04.b64" \
  "$script_dir/canonical-docs-ready.part05.b64" \
  "$script_dir/canonical-docs-ready.part06.b64" \
  "$script_dir/canonical-docs-ready.part07.b64" \
  > "$base64_stream"

python3 - "$base64_stream" "$expected_stream_sha256" <<'PY'
from pathlib import Path
import hashlib
import sys

path = Path(sys.argv[1])
expected = sys.argv[2]
actual = hashlib.sha256(path.read_bytes()).hexdigest()
if actual != expected:
    raise SystemExit(f"base64 stream SHA-256 mismatch: expected {expected}, got {actual}")
print(f"base64 stream SHA-256 OK: {actual}")
PY

if base64 --decode </dev/null >/dev/null 2>&1; then
  base64 --decode "$base64_stream" > "$archive"
else
  base64 -D "$base64_stream" > "$archive"
fi

python3 - \
  "$archive" \
  "$expected_archive_sha256" \
  "$expected_entries" \
  "$expected_uncompressed_bytes" \
  "$extract_dir" <<'PY'
from pathlib import Path
import hashlib
import sys
import zipfile

archive = Path(sys.argv[1])
expected_sha = sys.argv[2]
expected_entries = int(sys.argv[3])
expected_uncompressed = int(sys.argv[4])
extract_dir = Path(sys.argv[5])

actual_sha = hashlib.sha256(archive.read_bytes()).hexdigest()
if actual_sha != expected_sha:
    raise SystemExit(f"archive SHA-256 mismatch: expected {expected_sha}, got {actual_sha}")

with zipfile.ZipFile(archive) as zf:
    bad = zf.testzip()
    if bad is not None:
        raise SystemExit(f"ZIP CRC failure: {bad}")
    infos = zf.infolist()
    entries = len(infos)
    uncompressed = sum(info.file_size for info in infos)
    if entries != expected_entries:
        raise SystemExit(f"ZIP entry count mismatch: expected {expected_entries}, got {entries}")
    if uncompressed != expected_uncompressed:
        raise SystemExit(
            f"ZIP uncompressed-size mismatch: expected {expected_uncompressed}, got {uncompressed}"
        )
    names = [info.filename for info in infos]
    if not names or not all(name.startswith("canonical-docs/") for name in names):
        raise SystemExit("ZIP paths are not confined to canonical-docs/")
    for name in names:
        path = Path(name)
        if path.is_absolute() or ".." in path.parts:
            raise SystemExit(f"unsafe ZIP path: {name}")
    zf.extractall(extract_dir)

print(f"archive SHA-256 OK: {actual_sha}")
print(f"ZIP structure OK: {entries} entries, {uncompressed} uncompressed bytes")
print(f"extracted source: {extract_dir / 'canonical-docs'}")
PY
