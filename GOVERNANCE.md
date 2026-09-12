# Governance

Repository maintainers own technical direction for their scope. Cross-repository changes must identify the canonical owner of each interface and avoid duplicating sources of truth.

Material changes should record:
- the decision and alternatives considered;
- compatibility and migration consequences;
- security and operational risks;
- validation evidence;
- rollback or deprecation strategy.

Conflicted changes are merged by intent, not line selection. A semantic reconciliation should preserve non-conflicting additions from both sides and explicitly resolve incompatible assumptions.

## Legal documents

Legal templates, the agreement catalog, the service-tier and pricing data, the
TypeSpec + JSON Schema contracts that govern that data, and the registry
extracted from the templates all live in `canonical-docs`; the public prices
page in `canonical-cloud.github.io` renders a synced copy of the same data;
executed instruments live only in the governed records system and the
ores-legal signing platform. [`policies/LEGAL_DOCUMENT_GOVERNANCE.md`](policies/LEGAL_DOCUMENT_GOVERNANCE.md)
names the owner of each concern, the signing routes, the minimum coverage, and
the checks every legal change must pass; [`LEGAL_GOVERNANCE.md`](LEGAL_GOVERNANCE.md)
is the short organization standard, and the selectable
[`legal-contract-integrity`](workflow-templates/legal-contract-integrity.yml)
workflow template with the reusable
[`reusable-legal-contract-audit.yml`](.github/workflows/reusable-legal-contract-audit.yml)
workflow installs the contract checks in any repository that carries them.

<!-- ore-org-baseline:begin -->
## Sources of truth

- GitHub is authoritative for source, policy, architecture records, public organization context, reviewed implementation, and immutable commit history.
- [github.com/canonical-cloud](https://linear.app/denman/project/githubcomcanonical-cloud-1659c8ea1adf) is the planning and delivery ledger.
- Repository-local documentation is authoritative for repository-specific behavior and may strengthen this baseline.
- `repository-relationships.manual.json` is authoritative for reviewed public relationship declarations; the generated JSON graph is a deterministic projection.
- The approved private project registry is authoritative for private repository inventory and private-only edges.
- Private member context belongs in an approved private system, such as `.github-private`, never in this public repository.

## Change control

Material policy and architecture changes use issues or pull requests, focused commits, reviewable diffs, tests, and linked planning context. Existing content must be preserved unless a change explicitly supersedes it. Generated and mirrored artifacts must be updated from their authoritative source. Inferred relationship edges remain advisory until reviewed and declared.

Conflicts are resolved semantically with full history and cross-repository context. Destructive operations, history rewrites, force pushes, bypasses, and deletion of shared resources are default-deny and require exact authorization.

## Precedence

A repository may impose stricter requirements. It must not weaken secret handling, non-destructive collaboration, semantic conflict resolution, evidence-backed completion, or required review and checks.
<!-- ore-org-baseline:end -->
