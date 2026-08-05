# canonical-cloud organization handbook

> Shared operating defaults for repositories maintained under **canonical-cloud**. Repository-local policy may strengthen these rules but should not silently weaken them.

## Mission

canonical-cloud maintains reusable cloud infrastructure, automation, platform integrations, and operational tooling. This `.github` repository is the canonical home for organization-wide community health files, reusable templates, engineering policy, and planning links.

## Repository contract

Each active repository must document purpose, ownership, maturity, supported providers and environments, development and test commands, authoritative interfaces and configuration, deployment and rollback procedures, compatibility policy, and GitHub Project/Linear links. Infrastructure components must also document state ownership, credentials and trust boundaries, blast radius, idempotency, drift handling, observability, cost implications, disaster recovery, and provider limits.

## Change and review workflow

1. Anchor work in an issue, Linear item, or documented maintenance objective.
2. Keep branches and pull requests focused.
3. Explain motivation, scope, blast radius, validation, compatibility, migration, and rollback.
4. Test plan/dry-run, apply, retry, drift, partial failure, restore, and destroy protections as relevant.
5. Resolve conflicts semantically by reconstructing both sides' intent.
6. Prefer squash merges for focused work unless commit structure materially improves auditability.

## Evidence and quality

Pull requests should include reproducible commands, plan output or equivalent evidence, expected and observed results, failure-path coverage, documentation updates, and CI or local-equivalent evidence. Breaking infrastructure changes require consumer and environment impact analysis plus staged migration and rollback.

## Security and data

Never commit credentials, private keys, state files containing secrets, production data, or sensitive logs. Follow `SECURITY.md` for private vulnerability reporting. Use least privilege, pin dependencies/actions/images, verify provenance, and treat shared workflows as versioned APIs.

## Documentation and decisions

Keep examples executable and sanitized, links current, assumptions explicit, and ownership boundaries clear. Record architectural, provider, security, state, compatibility, cost, and operational decisions that future maintainers would otherwise have to rediscover.

## Planning ownership

GitHub owns code, reviews, checks, releases, and delivery evidence. Linear owns priority, dependencies, sequencing, and cross-project planning. The organization GitHub Project is the cross-repository execution view; see `PROJECTS.md` for routing details.

## Organization health

- [ ] Profiles, descriptions, topics, and READMEs are current.
- [ ] Contribution, security, support, governance, issue, and PR guidance is present.
- [ ] State, credentials, blast radius, rollback, recovery, and cost boundaries are documented.
- [ ] Required checks reflect infrastructure and supply-chain risk.
- [ ] Stale repositories are archived or clearly marked.
- [ ] Project links resolve and completed work is reflected in GitHub and Linear.
