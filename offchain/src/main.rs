use axum::{
    extract::{FromRef, Json},
    response::IntoResponse,
    routing::{get, post},
    Router,
};

use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod error;
mod handlers;
mod ipfs;
mod models;

use ipfs::IpfsClient;

// Application state
#[derive(Clone)]
pub struct AppState {
    pub ipfs_client: Arc<IpfsClient>,
}

// Implement FromRef to allow State extractor to work with AppState
impl FromRef<AppState> for Arc<IpfsClient> {
    fn from_ref(state: &AppState) -> Self {
        state.ipfs_client.clone()
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "offchain=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize IPFS client
    let ipfs_url =
        std::env::var("IPFS_URL").unwrap_or_else(|_| "http://127.0.0.1:5001".to_string());
    info!("Connecting to IPFS at: {}", ipfs_url);

    let ipfs_client = IpfsClient::new(&ipfs_url)?;

    // Create application state
    let state = AppState {
        ipfs_client: Arc::new(ipfs_client),
    };

    // Build router
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/v1/farmer/register", post(handlers::register_farmer))
        .route("/api/v1/fpo/purchase", post(handlers::fpo_purchase))
        .route("/api/v1/warehouse/update", post(handlers::warehouse_update))
        .route(
            "/api/v1/logistics/milestone",
            post(handlers::logistics_milestone),
        )
        .route("/api/v1/processing/batch", post(handlers::process_batch))
        .route("/api/v1/packaging/sku", post(handlers::create_sku))
        .route("/api/v1/ai/score", post(handlers::ai_score))
        .route("/api/v1/ipfs/upload", post(handlers::upload_to_ipfs))
        .route("/api/v1/ipfs/get/:cid", get(handlers::get_from_ipfs))
        .route("/api/v1/ipfs/pin/:cid", post(handlers::pin_ipfs))
        .with_state(state)
        .layer(CorsLayer::permissive());

    // Start server
    let addr = std::env::var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".to_string());
    info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "service": "offchain-ipfs-service",
        "version": env!("CARGO_PKG_VERSION")
    }))
}
