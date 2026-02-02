# Tech Pulse v2 — Architecture (Rust + Leptos)

This document defines the target architecture for the Rust-based system (backend + SSR web). It is intentionally forward-looking and can coexist with the current JS codebase.

---

## 1. Design Goals

1. **Extensibility**: Add sources, ranking strategies, or storage backends with minimal blast radius.
2. **SEO-first**: Public pages must be indexable (SSR/SSG support).
3. **Single-developer efficiency**: Clear module boundaries and predictable wiring.
4. **Scalable path**: Start as a modular monolith, evolve to multi-service when needed.

### Non-goals (for v2 MVP)
- Multi-region or multi-tenant deployments.
- Fully real-time streaming for ingestion.
- Heavy ML infrastructure (offline learning/embedding pipelines).

---

## 2. Workspace Layout (Recommended)

```
/apps
  /web            # Leptos SSR web app (SEO + PWA)
  /api            # JSON API service
  /worker         # Scheduled tasks: fetch, trend detection, ranking cache

/crates
  /domain         # Pure business rules & types (no IO)
  /usecase        # Application services (orchestration)
  /adapter        # Inbound adapters (HTTP, SSR, CLI)
  /infra          # Outbound adapters (DB, HTTP clients, cache)
  /shared         # Error types, config, tracing, time, ID

/schema
  /migrations     # DB migrations

/config
  dev.toml
  prod.toml

/docs
  ARCHITECTURE_V2.md
  ROADMAP.md

/assets
  /pwa            # manifest, icons
  /seo            # sitemap, robots

/infra
  /docker
  /deploy
```

---

## 3. Layering Rules (Dependency Direction)

Allowed dependency flow:

```
apps -> adapter -> usecase -> domain
apps -> adapter -> infra
usecase -> infra (via traits)
infra -> shared
adapter -> shared
```

Forbidden:
- `domain` must not depend on any other crate.
- `usecase` must not depend on `adapter` or app crates.
- `infra` must not depend on `adapter` or app crates.

This ensures core business logic remains stable while IO and transport can evolve independently.

---

## 4. Core Crates & Responsibilities

### `domain`
- Entities: Article, Topic, Trend, TimelineEvent, UserProfile
- Value objects: SourceId, ArticleScore, TimeWindow
- Policies: ranking weights, trend thresholds, blind-spot severity
- Traits (interfaces): ArticleRepo, ProfileRepo, TrendRepo, TimelineRepo

### `usecase`
- Application services (orchestration)
  - `FetchFeed`
  - `GenerateBriefing`
  - `DetectTrends`
  - `UpdateProfileFromInteractions`
  - `GenerateTimeline`
- Only calls domain + repository traits

### `adapter`
- Inbound boundaries:
  - HTTP handlers (JSON API)
  - SSR route handlers (Leptos integration)
  - CLI commands (dev/admin)
- DTO mapping, request validation, response shaping

### `infra`
- Outbound boundaries:
  - DB implementations (SQLite -> Postgres later)
  - External source clients (HN/Reddit/GitHub/etc.)
  - Caching (in-memory/redis)
  - Translation providers

### `shared`
- Error types, result helpers
- Config parsing & env override
- Tracing/logging
- Time/clock abstraction (testable)

---

## 5. App Services

### `apps/api`
- JSON API for clients and SSR
- Calls `usecase` directly
- Stateless; uses shared repo implementations from `infra`

### `apps/web`
- Leptos SSR for public pages
- PWA manifest and offline strategy
- Can call `usecase` directly (same process) or call `apps/api` over HTTP
  - **Default**: same process in MVP for simplicity
  - **Future**: split for scalability

### `apps/worker`
- Scheduled jobs:
  - Fetch all sources every 30 minutes
  - Compute trends (6h window)
  - Refresh ranking caches
- Writes to DB and cache only; no HTTP exposure

---

## 6. Data Flow (High Level)

```
Sources -> worker (fetch) -> DB -> usecase
                               |-> trend detection -> trends_cache
API/Web -> usecase -> DB/cache -> feed response
User interaction -> API -> usecase -> profile updates -> DB
```

---

## 7. Extensibility Playbooks

### Add a new data source
1. Implement a new `SourceClient` in `infra`.
2. Map raw fields to `domain::Article`.
3. Register in `worker` ingestion pipeline.

### Add a new ranking factor
1. Define the factor in `domain` (policy & weight).
2. Update `usecase::FetchFeed` scoring logic.
3. Add metrics to observe impact.

### Swap SQLite for Postgres
1. Implement repository traits in `infra` for Postgres.
2. Adjust config to select backend.
3. Run migrations from `/schema/migrations`.

---

## 8. Configuration & Observability

- All config in `/config/*.toml`, overridden by env vars.
- Use `tracing` for structured logs.
- Include request IDs and source IDs in logs for diagnosis.

---

## 9. Security & Privacy (MVP)

- Anonymous device token for identification.
- Interaction events stored with minimal PII.
- Keep data retention policies configurable.

---

## 10. Migration Strategy

The existing JS architecture remains as v1. This document defines the v2 target. Migration proceeds by:
1. Build Rust API + worker first.
2. Switch front-end to SSR Leptos once API is stable.
3. Retire JS adapters and CORS proxies last.

---

## 11. Testing Strategy

- `domain`: unit tests for policies and scoring.
- `usecase`: integration tests with in-memory repos.
- `infra`: contract tests for adapters and DB.
- `apps`: basic HTTP/SSR smoke tests.

---

## 12. Non-Negotiable Invariants

1. **Domain stays pure** (no IO, no infra imports).
2. **All external API parsing is in `infra`** (never in `usecase`).
3. **All ranking logic is centralized** (no ad-hoc scoring in adapters).
4. **All migrations are applied via schema tooling** (no ad-hoc SQL at runtime).

