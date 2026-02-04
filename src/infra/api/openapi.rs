//! OpenAPI specification for Tournamint API.
//!
//! Served at `/api-docs/openapi.json` and displayed via Swagger UI at `/swagger-ui/`.
//! Add more paths in the `paths` module to expand documentation.

mod paths;

use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};

struct SecurityModifier;

impl Modify for SecurityModifier {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_with(Default::default);
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .description(Some("JWT from Google OAuth callback"))
                    .build(),
            ),
        );
    }
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Tournamint API",
        version = "0.1.0",
        description = "Tournament management API — auth, users, tournaments, matches, brackets, standings, payments, notifications, statistics.",
        license(name = "MIT")
    ),
    servers(
        (url = "http://localhost:8080", description = "Local development")
    ),
    paths(
        paths::auth::start_google_login,
        paths::auth::google_callback,
        paths::users::list_users,
        paths::users::get_user,
        paths::profile::get_current_profile,
        paths::profile::get_public_profile,
        paths::tournaments::list_tournaments,
        paths::tournaments::get_tournament,
        paths::tournaments::create_tournament,
        paths::tournaments::get_upcoming,
        paths::tournaments::search,
        paths::brackets::get_bracket_by_category,
        paths::brackets::generate_bracket,
        paths::standings::get_standings_by_category,
        paths::matches::get_match,
        paths::matches::list_live_matches,
        paths::match_results::create_match_result,
    ),
    tags(
        (name = "Auth", description = "Google OAuth authentication"),
        (name = "Users", description = "User management"),
        (name = "Profile", description = "User profiles"),
        (name = "Tournaments", description = "Tournament CRUD and queries"),
        (name = "Brackets", description = "Bracket generation and retrieval"),
        (name = "Standings", description = "Tournament standings"),
        (name = "Matches", description = "Match management"),
        (name = "Match Results", description = "Match result entry")
    ),
    modifiers(&SecurityModifier),
    security(
        ("bearer_auth" = [])
    )
)]
pub struct ApiDoc;
