# Tournament Server - Setup Instructions

## Current Status
✅ All 7 missing API features have been implemented
✅ ApiResponse utility has been created
✅ All compilation issues have been fixed (except database-related)

## Setup Steps

### 1. Database Setup
The server requires PostgreSQL with the database tables created. The tables `matches` and `match_results` already exist from previous migrations.

```bash
# 1. Ensure PostgreSQL is running
# 2. Create database if not exists
createdb tournamint

# 3. Update .env with your database URL
DATABASE_URL=postgres://username:password@localhost/tournamint

# 4. Run migrations
sqlx migrate run
```

### 2. Environment Variables
Copy `.env.example` to `.env` and fill in:
```
APP_URL=127.0.0.1
APP_PORT=8080
DATABASE_URL=postgres://your_user:your_password@localhost/tournamint
GOOGLE_CLIENT_ID=your_google_client_id
GOOGLE_CLIENT_SECRET=your_google_client_secret
GOOGLE_REDIRECT_URL=http://localhost:8080/auth/google/callback
JWT_SECRET=your_jwt_secret_key
```

### 3. Build and Run

#### Option A: With database available
```bash
cargo build
cargo run
```

#### Option B: Without database (offline mode)
```bash
# Generate sqlx offline data first (requires database to be available once)
cargo sqlx prepare

# Then you can build offline
SQLX_OFFLINE=true cargo build
SQLX_OFFLINE=true cargo run
```

## API Endpoints

- **Markdown docs**: `server/docs/API_DOCUMENTATION.md`
- **OpenAPI / Swagger UI** (when server is running): `http://localhost:8080/swagger-ui/`
- **OpenAPI JSON**: `http://localhost:8080/api-docs/openapi.json`

Swagger UI supports JWT: use **Authorize** and enter `Bearer <your_jwt_token>` to test authenticated endpoints.

### Quick Test
Once running, test the health of the server:
```bash
# Get JWT token first (through Google OAuth)
# Then test an endpoint
curl -H "Authorization: Bearer YOUR_JWT_TOKEN" http://localhost:8080/matches
```

## Implemented Features

1. ✅ **Match Management** - Full CRUD for matches
2. ✅ **Match Results** - Score tracking and results
3. ✅ **Tournament Brackets** - Bracket generation and management
4. ✅ **Tournament Standings** - Rankings and standings
5. ✅ **User Profiles** - Profile management with preferences
6. ✅ **Notifications** - Real-time notifications system  
7. ✅ **Payments** - Payment processing with refunds
8. ✅ **Statistics/Analytics** - Comprehensive stats and analytics

## Known Issues

The project will show sqlx compilation errors if:
- Database is not available
- Migrations haven't been run
- SQLX_OFFLINE mode is not configured

These are not actual code errors - they're database connection requirements from sqlx's compile-time checking.

## Development Notes

- All handlers use the `ApiResponse` utility in `src/shared/api_response.rs`
- Authentication is handled via JWT tokens in the Authorization header
- All endpoints except public profiles require authentication
- The codebase follows DDD architecture (domain, application, infra) with clean separation of concerns