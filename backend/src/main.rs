// axum-server/src/main.rs

use axum::{
    http::{header::CONTENT_TYPE, Method},
    routing::post,
    Json, Router,
};

use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tracing::{debug, error, info, instrument};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize tracing for structured, leveled logging.
    // This allows you to control log verbosity via the `RUST_LOG` environment variable.
    // For example: `RUST_LOG=debug cargo run -p axum-server`
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axum_server=info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("initializing CORS layer");
    // Define the Cross-Origin Resource Sharing (CORS) policy.
    // This is crucial for allowing the SvelteKit frontend (on a different port)
    // to communicate with this server.
    let cors = CorsLayer::new()
        .allow_origin(Any) // Allow any origin for development convenience.
        .allow_methods([Method::OPTIONS, Method::POST]) // Allow POST and the preflight OPTIONS request.
        .allow_headers([CONTENT_TYPE]); // Allow the frontend to send the Content-Type header.

    info!("defining application routes");
    // Define the application router and its single API endpoint.
    let app = Router::new()
        .route("", get(|| async { "Welcome to the Poker Math Quiz Game API!" }))
        .layer(cors);

    // Bind to the socket address.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = match TcpListener::bind(addr).await {
        Ok(listener) => {
            info!("listening on http://{}", addr);
            listener
        }
        Err(e) => {
            // If we can't bind to the port, log a fatal error and exit.
            error!("failed to bind to address {}: {}", addr, e);
            return;
        }
    };

    // Serve the application with a graceful shutdown signal.
    info!("starting server");
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

/// Helper function to create a future that resolves when the server should shut down.
/// Listens for a Ctrl+C signal on all platforms. On Unix-like systems, it also
/// listens for the TERM signal (used by services like systemd).
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            info!("received Ctrl+C, starting graceful shutdown");
        },
        _ = terminate => {
            info!("received terminate signal, starting graceful shutdown");
        },
    }
}