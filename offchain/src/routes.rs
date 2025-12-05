use crate::handlers;
use axum::{
    routing::{get, post},
    Router,
};

pub fn configure_routes() -> Router {
    Router::new()
    // .route("/api/example/:id", get(handlers::get_example))
    // .route("/api/example", post(handlers::post_example))
    // .route("/api/error", get(handlers::error_example))
}
