// API infrastructure - HTTP handlers, routes, middleware

pub mod handlers;
pub mod middleware;
pub mod openapi;
pub mod routes;

pub use routes::api_routes;
