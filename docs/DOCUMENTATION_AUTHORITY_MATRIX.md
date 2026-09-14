# Canonical Cloud documentation authority matrix

This document records which source wins when Canonical Cloud documentation, implementation notes, generated artifacts, and planning material disagree. It is a routing map for evidence and ownership; it is **not** a new wire-contract, legal, commercial, or deployment authority.

A lower row must not silently override a higher-level authority for a different concern. When two listed authorities conflict within the same concern, open an explicit decision issue and keep the conflict visible until it is resolved.

## Authority matrix

| Concern | Primary authority | Consumers / projections | Must not be inferred |
| --- | --- | --- | --- |
| Public claims and evidence classification | `canonical-docs/docs/claims-register.md` | marketing, sales docs, public sites, README copy | source code alone proves production deployment or customer outcomes |
| Product positioning, public information architecture and browser-origin policy | `canonical-docs/docs/market-positioning.md` | marketing site, public docs, web redirects/origins, publication checklist | legal entity/domain automatically changes with product branding |
| Legal entity, contract text and executed customer obligations | reviewed/executed legal agreements; draft corpus in `canonical-docs/docs/legal/` is preparation material only | customer/legal operations | a README, market-positioning page or source repository changes an executed agreement |
| Readiness package planning price bands | `canonical-docs/docs/business-plan.md` + admitted readiness-offering peer contract/projection | package docs, marketing planning copy | a planning table overrides a signed SOW |
| Engagement-specific commercial scope/price/term | signed customer SOW | delivery workflow | website/catalog planning values are binding terms |
| Readiness assessment wire semantics | independent TypeSpec + authored Draft 2020-12 JSON Schema peer authorities in `canonical-interfaces` admitted by TJSV | admitted Contract IR and generated adapters | generated schema/adapter output is a third editable authority |
| Readiness workspace/publication data semantics | peer authorities in `canonical-interfaces` | API, CLI, clients, ORM, web, e2e | provider storage layout becomes customer wire semantics |
| HTTP/RPC route names, methods and paths | `canonical-interfaces/route-maps/*` and their admitted generator | API/web/CLI/SDK/docs | a service README or literal client URL creates a parallel production route authority |
| Generated language adapters | admitted source contracts/Contract IR + generator | Rust, Rust-WASM, TypeScript, Python, Go, Dart consumers | hand edits resolve drift |
| API documentation | generated/validated from the same route and message contracts | customer/internal API docs | prose API examples outrank compiled route/type contracts |
| Customer tenant authorization | shared-auth verified principal + server authorization policy | API, web, clients, MCP | body/query/path/header tenant identifiers are authority |
| Admin authorization | isolated admin service policy, explicit capability + step-up requirements | admin web/API | customer roles or customer-plane credentials imply admin access |
| Persistence schema and migrations | the owning persistence/service declarative migration source, with portable implementation in `canonical-orm-core` where shared | Supabase/Neon deployment targets | a database provider becomes business/wire-contract authority |
| Relational write authority | one documented authoritative writer + fencing epoch per logical dataset/environment | Supabase/Neon promotion/reconciliation | both providers may independently accept the same logical writes |
| Offline/sync authority | server contract + `canonical-sync`/opto-sync policy for explicitly synchronizable records | browser/mobile/desktop caches | cached/offline state can rewrite immutable publication records or bypass authorization |
| Object/report content storage | API-authorized publication policy + `canonical-infra` private R2 implementation | web/API retrieval and reconciliation | R2 key, bucket or signed URL is tenant authorization |
| Deployment/provider topology | `canonical-infra` plus the deployable service runtime contracts | Cloud Run, `ORESoftware/k8s-cluster`, Cloudflare | provider-specific topology changes application semantics |
| Release composition/pinned application revisions | `canonical-monorepo` gitlinks/release manifest | deploy pipelines | the monorepo owns copied child-repo source or rewrites child contracts |
| Customer runtime responsibilities | dedicated customer web/API repositories | infra, clients, monorepo | customer web gets privileged DB/R2/admin mutation authority |
| Admin runtime responsibilities | dedicated admin web/API repositories | infra, monorepo | admin code is merely a future module inside the customer server |
| Agent/tool exposure | `canonical-mcp-server.rs` for non-admin tools plus shared authorization | MCP clients/agents | tool arguments establish tenant/admin authority |
| Cross-repo engineering policy and exception registry | `canonical-cloud/.github` | all repos selectively | every repo must carry service-only dependencies or identical controls |
| Company/product/operating narrative | `canonical-docs` | public/internal planning surfaces | prose becomes executable policy where `.github`, contracts or runtime code own enforcement |
| Production-operating evidence | deployed runtime/CI/observability/operating records for the exact artifact/environment | claims register and runbooks | repository presence or a green unrelated workflow proves deployment |

## Current known conflicts requiring explicit resolution

### Brand, domain and browser origins

The current corpus is not internally consistent:

- `docs/market-positioning.md` is marked active and assigns `canonical.plus` to marketing, `app.canonical.plus` to the single authenticated browser/session origin, `account.canonical.plus` to redirect-only use and `api.canonical.plus` to a future API origin;
- `canonical-docs/README.md` and the business plan still identify `canonical.cloud` as the primary/public domain;
- `docs/legal/audit-config.json` records the legal entity as `Canonical Cloud, Inc.` with domain `canonical.cloud`;
- `canonical-web-server.rs/README.md` still describes the customer server as the application server for `canonical.cloud`;
- some route fixtures name `user.canonical.plus` and `org.canonical.plus`, which are not part of the active market-positioning host inventory.

Do **not** fix this by mechanically replacing domains. The legal entity, product brand, marketing host, authenticated host, API host, email domain, repository names, cookie origins and redirect/deprecation policy are distinct decisions. Resolve them explicitly, then synchronize consumers and counsel-reviewed legal material where applicable.

### Readiness route prefixes and aliases

The shared API route map currently uses `/api/v1/readiness/...` for authenticated readiness routes. Some docs and draft clients still use `/v1/readiness/...`; the continuous-readiness webhook draft documents both prefixes as production endpoints.

One shared route authority must define the canonical path. Compatibility aliases, if intentionally retained, must be explicit generated compatibility metadata with deprecation/removal policy rather than prose-only alternate authorities.

### Extension headers

Current source/doc examples include `X-Canonical-*`, `x-canonical-subject`, standard `Idempotency-Key`, and ecosystem-wide ORESoftware extension conventions. Authored HTTP header names must be lowercase. ORESoftware-owned extension headers should normally use the reserved `x-ores-*` namespace; any intentionally public `x-canonical-*` protocol namespace must be documented as a deliberate exception with translation/ingress-spoofing rules rather than emerging independently in each repo.

### Repository inventory and source-of-truth terminology

`canonical-docs/docs/repository-boundaries.md` currently covers only a subset of the active organization and uses older wording that describes JSON Schema/SQL/generated adapters as a generic source of truth. The current contract model instead uses independent TypeSpec + authored JSON Schema peer authorities, TJSV admission, read-only Contract IR/generated projections, and separate storage/migration authority.

The inventory must be expanded to the current repository set and distinguish wire contracts, route authority, persistence migrations, generated evidence, release pins and deployment configuration.

### Pricing documents

Readiness package planning values have a defined hierarchy: management planning source + admitted offer contract/projection, with the signed SOW controlling the actual engagement. Existing legal service-tier/pricing documents must not accidentally become a second readiness-package price authority. Resolve known price conflicts through the existing commercial/legal review work rather than making an engineering-only pricing choice.

## Change rule

When a change affects more than one authority row:

1. change the actual owning authority first;
2. admit/test it at that layer;
3. regenerate or update consumers;
4. update explanatory docs and this matrix if ownership changed;
5. record a deliberate exception when a repo must differ;
6. require exact-head evidence for the changed enforcement layer.

A stale document should be fixed, but a stale document must never be used as justification to overwrite a newer executable authority without semantic review.
