use axum::Router;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::routes::create_routers;

mod handlers;
mod models;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    // initialize tracing with debug level to capture HTTP logs (can be overridden with RUST_LOG env var)
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("debug")),
        )
        .init();

    info!("🚀 Starting weeb-site server...");

    // build our application with a route
    let app = Router::new()
        .merge(create_routers())
        .nest_service("/static", ServeDir::new("src/static"))
        .layer(TraceLayer::new_for_http());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    info!("✅ Server running on http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}
