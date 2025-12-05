use axum::{routing::get, Router};
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod error;
mod handlers;
mod models;
mod routes;

use config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "offchain=debug,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenvy::dotenv().ok();

    // Load configuration
    let config = Config::from_env()?;

    tracing::info!(
        "Starting server in {:?} mode on {}",
        config.environment,
        config.address()
    );

    // Build router
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .merge(routes::configure_routes())
        .layer(CorsLayer::permissive());

    let addr = config.address();
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    tracing::info!("Server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> &'static str {
    "Backend API is running"
}

async fn health_check() -> &'static str {
    "OK"
}
