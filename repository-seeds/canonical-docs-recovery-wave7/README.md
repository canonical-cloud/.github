# Canonical Docs recovery source — Wave 7

This directory preserves the verified source tree recovered for the still-missing public repository `canonical-cloud/canonical-docs`.

The retained recovery artifact does **not** contain the original `.git` object database. Accordingly, this is an exact source/archive recovery, not a claim that any reconstructed commit ID is an original historical commit. The target repository must be created through an organization-capable repository-administration lane tracked by `DEN-319`; after creation, this source can be committed and pushed normally without force-pushing or rewriting another repository's history.

## Evidence

- archive: `canonical-docs-ready.zip`
- archive bytes: `45145`
- archive SHA-256: `6afa4bef55c3b69b22cc1cad0468d156bce7e7413f3af47addbcca4b25c811c4`
- ZIP entries: `36`
- uncompressed bytes: `103011`
- recovered document contract: PASS
- recovered unit tests: **11/11 PASS**
- credential-shape scan of the mounted recovery tree: no hit

The archive is stored as eight ordered base64 segments because the connected GitHub write surface accepts textual files. `manifest.json` pins every segment by size, SHA-256, and Git blob object ID.

## Reconstruct and verify

```bash
bash ./reconstruct.sh /tmp/canonical-docs-wave7
```

The script:

1. verifies the ordered concatenated base64 stream;
2. decodes the exact ZIP;
3. verifies the ZIP SHA-256;
4. checks the ZIP structure, entry count, and uncompressed size; and
5. extracts the `canonical-docs/` tree.

Manual equivalent:

```bash
cat canonical-docs-ready.part*.b64 > canonical-docs-ready.zip.b64
base64 --decode canonical-docs-ready.zip.b64 > canonical-docs-ready.zip
printf '%s  %s\n' \
  6afa4bef55c3b69b22cc1cad0468d156bce7e7413f3af47addbcca4b25c811c4 \
  canonical-docs-ready.zip | sha256sum --check
unzip canonical-docs-ready.zip
```

On macOS, use `base64 -D` instead of `base64 --decode`.

## Publication contract

1. Create `canonical-cloud/canonical-docs` as an empty public repository.
2. Reconstruct and validate this source tree.
3. Initialize `main`, commit the recovered source with an explicit message such as `reconstruct verified canonical-docs source from Wave 7`, and push normally.
4. Run `python -m unittest discover -s tests -v` and `python scripts/check_docs.py` at the published head.
5. Record the newly reconstructed commit ID as a **new** commit, never as an alleged original recovery head.

No force push, rebase, history replacement, token persistence, or claim of target-repository creation is made here.

Tracking: `DEN-1049`, `DEN-2797`, and repository-administration blocker `DEN-319`.
