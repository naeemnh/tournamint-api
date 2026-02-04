//! OpenAPI path stubs for API documentation.
//!
//! These functions are not called; they exist only for utoipa to generate the spec.
//! To add a new endpoint: add a `#[utoipa::path(...)]` stub here, then register it
//! in the `paths(...)` list in `openapi.rs`.

pub mod auth {
    #[utoipa::path(
        get,
        path = "/auth/google",
        responses((status = 302, description = "Redirects to Google OAuth consent")),
        tag = "Auth"
    )]
    #[allow(dead_code)]
    pub fn start_google_login() {}

    #[utoipa::path(
        get,
        path = "/auth/google/callback",
        params(("code" = String, Query, description = "Authorization code from Google")),
        responses((status = 200, description = "Returns JWT and user")),
        tag = "Auth"
    )]
    #[allow(dead_code)]
    pub fn google_callback() {}
}

pub mod users {
    #[utoipa::path(
        get,
        path = "/users",
        responses((status = 200, description = "List users")),
        tag = "Users"
    )]
    #[allow(dead_code)]
    pub fn list_users() {}

    #[utoipa::path(
        get,
        path = "/users/{id}",
        params(("id" = uuid::Uuid, Path, description = "User ID")),
        responses((status = 200, description = "User details"), (status = 404, description = "Not found")),
        tag = "Users"
    )]
    #[allow(dead_code)]
    pub fn get_user() {}
}

pub mod profile {
    #[utoipa::path(
        get,
        path = "/profile",
        responses((status = 200, description = "Current user profile")),
        tag = "Profile"
    )]
    #[allow(dead_code)]
    pub fn get_current_profile() {}

    #[utoipa::path(
        get,
        path = "/profile/{user_id}",
        params(("user_id" = uuid::Uuid, Path, description = "User ID")),
        responses((status = 200, description = "Public profile")),
        tag = "Profile"
    )]
    #[allow(dead_code)]
    pub fn get_public_profile() {}
}

pub mod tournaments {
    #[utoipa::path(
        get,
        path = "/tournaments",
        responses((status = 200, description = "List tournaments")),
        tag = "Tournaments"
    )]
    #[allow(dead_code)]
    pub fn list_tournaments() {}

    #[utoipa::path(
        get,
        path = "/tournaments/{id}",
        params(("id" = uuid::Uuid, Path, description = "Tournament ID")),
        responses((status = 200, description = "Tournament details"), (status = 404, description = "Not found")),
        tag = "Tournaments"
    )]
    #[allow(dead_code)]
    pub fn get_tournament() {}

    #[utoipa::path(
        post,
        path = "/tournaments",
        responses((status = 201, description = "Tournament created")),
        tag = "Tournaments"
    )]
    #[allow(dead_code)]
    pub fn create_tournament() {}

    #[utoipa::path(
        get,
        path = "/tournaments/upcoming",
        responses((status = 200, description = "Upcoming tournaments")),
        tag = "Tournaments"
    )]
    #[allow(dead_code)]
    pub fn get_upcoming() {}

    #[utoipa::path(
        get,
        path = "/tournaments/search",
        responses((status = 200, description = "Search results")),
        tag = "Tournaments"
    )]
    #[allow(dead_code)]
    pub fn search() {}
}

pub mod brackets {
    #[utoipa::path(
        get,
        path = "/brackets/category/{category_id}",
        params(("category_id" = uuid::Uuid, Path, description = "Category ID")),
        responses((status = 200, description = "Bracket for category")),
        tag = "Brackets"
    )]
    #[allow(dead_code)]
    pub fn get_bracket_by_category() {}

    #[utoipa::path(
        put,
        path = "/brackets/generate/{tournament_id}",
        params(("tournament_id" = uuid::Uuid, Path, description = "Tournament ID")),
        responses((status = 200, description = "Bracket generated")),
        tag = "Brackets"
    )]
    #[allow(dead_code)]
    pub fn generate_bracket() {}
}

pub mod standings {
    #[utoipa::path(
        get,
        path = "/standings/category/{category_id}",
        params(("category_id" = uuid::Uuid, Path, description = "Category ID")),
        responses((status = 200, description = "Standings for category")),
        tag = "Standings"
    )]
    #[allow(dead_code)]
    pub fn get_standings_by_category() {}
}

pub mod matches {
    #[utoipa::path(
        get,
        path = "/matches/{id}",
        params(("id" = uuid::Uuid, Path, description = "Match ID")),
        responses((status = 200, description = "Match details"), (status = 404, description = "Not found")),
        tag = "Matches"
    )]
    #[allow(dead_code)]
    pub fn get_match() {}

    #[utoipa::path(
        get,
        path = "/matches/live",
        responses((status = 200, description = "Live matches")),
        tag = "Matches"
    )]
    #[allow(dead_code)]
    pub fn list_live_matches() {}
}

pub mod match_results {
    #[utoipa::path(
        post,
        path = "/match-results",
        responses((status = 201, description = "Match result created")),
        tag = "Match Results"
    )]
    #[allow(dead_code)]
    pub fn create_match_result() {}
}
