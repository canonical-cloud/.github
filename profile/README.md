# canonical-cloud

This organization maintains software, infrastructure, interfaces, clients, services, and supporting documentation under a shared engineering baseline.

## Working principles

- Keep changes reviewable, tested, and reversible.
- Treat security, privacy, compatibility, and data durability as design constraints.
- Resolve merge conflicts semantically: reconstruct both sides' intent, preserve compatible behavior, and document deliberate trade-offs.
- Prefer canonical repositories and short, stable names; deprecate duplicates with migration notes rather than silently deleting history.
- Keep cross-repository dependencies explicit and pinned where reproducibility matters.

Organization-wide contribution and security guidance lives in this `.github` repository.

<!-- org-project-routing:start -->
## Planning and delivery

- [GitHub Project: canonical-cloud-project](https://github.com/orgs/canonical-cloud/projects/1)
- [Linear planning project](https://linear.app/denman/project/githubcomcanonical-cloud-1659c8ea1adf)
- [Detailed project-routing contract](../docs/PROJECTS.md)

GitHub owns code and delivery evidence; Linear owns planning and dependencies. The linked organization Project provides the cross-repository execution view.
<!-- org-project-routing:end -->

## Active delivery documentation

- [Authenticated compliance quote delivery map — August 5, 2026](../docs/AUTHENTICATED-QUOTE-DELIVERY.md)

<!-- ore-org-baseline:begin -->
## Planning and governance

- Canonical Linear project: https://linear.app/denman/project/githubcomcanonical-cloud-1659c8ea1adf
- Organization defaults: https://github.com/canonical-cloud/.github
- Canonical agent policy: https://github.com/canonical-cloud/.github/blob/main/agents.md
- Security policy: https://github.com/canonical-cloud/.github/security/policy

Repositories in this organization use semantic conflict resolution with 3–10 relevant prior commits when useful, full cross-repository context, pull-request delivery, and a hard automated-agent denylist for destructive or history-rewriting operations.
<!-- ore-org-baseline:end -->
