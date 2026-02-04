# Tournamint Backend (Server)

Rust/Actix-web API for the Tournamint tournament management system. The backend uses a **Domain-Driven Design (DDD)** architecture: domain → application (use cases) → infra (HTTP handlers, PostgreSQL repositories).

## Quick start

1. Copy `server/.env.example` to `server/.env` and set `DATABASE_URL`, `GOOGLE_*`, `JWT_SECRET`, etc.
2. From `server/`: `sqlx migrate run` then `cargo run`.
3. Server listens on `http://127.0.0.1:8080` (or `APP_URL:APP_PORT`).
4. **OpenAPI / Swagger UI**: `http://127.0.0.1:8080/swagger-ui/` for interactive API docs; `http://127.0.0.1:8080/api-docs/openapi.json` for the OpenAPI spec.

See [docs/backend-setup.md](../docs/backend-setup.md) in the project root for full setup.

## Implemented APIs (DDD coverage)

All documented endpoints in [docs/backend-api-reference.md](../docs/backend-api-reference.md) are implemented via use cases and handlers:

| Area | Scope | Notes |
|------|--------|--------|
| **Auth** | `/auth/google` | Google OAuth, JWT |
| **Users** | `/users` | CRUD |
| **User profile** | `/profile` | Current profile, update, preferences, notifications, privacy, avatar; public profile by `user_id` |
| **Players** | `/players` | CRUD |
| **Teams** | `/teams` | CRUD |
| **Team members** | `/team_members` | Add, get by team/player, update/delete (composite path) |
| **Tournaments** | `/tournaments` | CRUD, search, status, my, featured, upcoming, templates, lifecycle, stats, export, duplicate, dashboard, settings |
| **Tournament categories** | `/tournament_categories` | Create, get by id/tournament, update, delete |
| **Tournament registrations** | `/tournament_registrations` | Full CRUD + by category/tournament/player/team |
| **Brackets** | `/brackets` | By tournament/category, generate |
| **Standings** | `/standings` | By tournament/category, update |
| **Matches** | `/matches` | CRUD, by tournament/category, participants, status/lifecycle, schedule, my/upcoming/history, live, analytics, media, comments, subscribe, bulk |
| **Match results** | `/match-results` | CRUD, by match (list/summary/count/set), delete all, bulk create |
| **Notifications** | `/notifications` | List, unread, count, read-all, send, mark read, delete |
| **Payments** | `/payments` | Process, get by id/user/tournament, refund, status, summaries |
| **Statistics** | `/stats` | Player/team/tournament stats, leaderboards, records, summary, my-stats |
| **Analytics** | `/analytics` | Dashboard, growth |

## Project layout

- **`src/domain/`** — Entities, value objects, repository traits (no SQLx/Actix).
- **`src/application/`** — Use cases (auth, user, participant, tournament, match, notification, payment, statistics).
- **`src/infra/db/`** — PostgreSQL repository implementations (SQLx).
- **`src/infra/api/`** — Routes and HTTP handlers; handlers call use cases. OpenAPI spec and Swagger UI are in `openapi/` and `openapi.rs`.
- **`src/shared/`** — Config, errors, API response helpers, JWT, Google OAuth.
- **`migrations/`** — SQLx migrations.

## Documentation

- **Project docs** (root `docs/`): [backend-overview.md](../docs/backend-overview.md), [backend-api-reference.md](../docs/backend-api-reference.md), [backend-setup.md](../docs/backend-setup.md), [backend-database.md](../docs/backend-database.md).
- **Server-local**: `server/docs/API_DOCUMENTATION.md`, `server/SETUP_INSTRUCTIONS.md`.
- **OpenAPI**: Add path stubs in `src/infra/api/openapi/paths.rs` and register them in `openapi.rs` to extend Swagger docs.

## Tests

From `server/`: `cargo test`. Some tests require a database; see `server/tests/` and `server/tests/unit/`.
