use crate::handlers;
use axum::{
    routing::{get, post},
    Router,
};

/// Configure all application routes
pub fn configure_routes() -> Router {
    Router::new()
        // Example routes - replace with your own
        .route("/api/example/:id", get(handlers::get_example))
        .route("/api/example", post(handlers::post_example))
        .route("/api/error", get(handlers::error_example))
}
