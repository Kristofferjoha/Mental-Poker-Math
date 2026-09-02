use axum::{Router, routing::{get, post}};
use tower_http::cors::{AllowOrigin, CorsLayer};
use std::{net::SocketAddr, sync::Arc, time::Duration, env};
use tracing::{info, warn};
use poker_eval::eval::seven as seven_eval;
use moka::sync::Cache;
use axum::http::{header, HeaderValue, Method};

use crate::utils::{api, app_state::AppState};


const DEFAULT_ALLOWED_ORIGINS: &[&str] = &[
    "https://mentalpokermath.com",
    "https://www.mentalpokermath.com",
];

pub fn is_localhost_origin(origin: &HeaderValue) -> bool {
    let Ok(text) = origin.to_str() else {
        return false;
    };
    let Some(authority) = text.strip_prefix("http://") else {
        return false;
    };
    matches!(
        authority.split(':').next(),
        Some("localhost") | Some("127.0.0.1")
    )
}

fn validate_origin(entry: &str) -> Result<HeaderValue, String> {
    let host = entry
        .strip_prefix("https://")
        .or_else(|| entry.strip_prefix("http://"))
        .ok_or("must start with http:// or https://")?;

    if host.is_empty() {
        return Err("has no host".to_string());
    }
    if entry.contains(char::is_whitespace) {
        return Err("contains whitespace".to_string());
    }
    if host.contains('/') {
        return Err("must not contain a path or trailing slash".to_string());
    }

    entry
        .parse::<HeaderValue>()
        .map_err(|e| format!("not a valid header value: {e}"))
}

pub fn allowed_origins() -> anyhow::Result<Vec<HeaderValue>> {
    let raw = env::var("CORS_ALLOWED_ORIGINS")
        .unwrap_or_else(|_| DEFAULT_ALLOWED_ORIGINS.join(","));

    let origins: Vec<HeaderValue> = raw
        .split(',')
        .map(str::trim)
        .filter(|entry| !entry.is_empty())
        .filter_map(|entry| match validate_origin(entry) {
            Ok(value) => Some(value),
            Err(reason) => {
                warn!("Ignoring invalid CORS origin {:?}: {}", entry, reason);
                None
            }
        })
        .collect();

    if origins.is_empty() {
        anyhow::bail!("CORS allow-list is empty; no browser origin could reach the API");
    }

    Ok(origins)
}

pub fn build_router(app_state: AppState) -> Router {
    Router::new()
        .route("/api/pot-equity-get-problem", get(api::generate_pot_equity_problem))
        .route("/api/pot-equity-check-answer", post(api::check_pot_equity_answer))
        .route("/api/pure-equity-get-problem", get(api::generate_pure_equity_problem))
        .route("/api/pure-equity-check-answer", post(api::check_pure_equity_answer))
        .route("/api/whats-the-nuts-get-problem", get(api::generate_nuts_problem))
        .route("/api/whats-the-nuts-check-answer", post(api::check_nuts_answer))
        .with_state(app_state)
}

/// Main entrypoint for the Axum application.
/// Sets up state, routes, and starts the HTTP server.
pub async fn run() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let host_str = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port_str = env::var("PORT").unwrap_or_else(|_| "8001".to_string());
    let addr_str = format!("{}:{}", host_str, port_str);
    let addr: SocketAddr = addr_str.parse().expect("Unable to parse socket address");

    info!("Building 7-card lookup tables...");
    let seven_card_tables = Arc::new(seven_eval::build_tables(false));
    info!("Tables built successfully.");


    const CACHE_TTL: Duration = Duration::from_secs(60 * 60);
    const CACHE_MAX_CAPACITY: u64 = 50_000;

    let app_state = AppState {
        pot_equity_cache: Arc::new(
            Cache::builder()
                .time_to_live(CACHE_TTL)
                .max_capacity(CACHE_MAX_CAPACITY)
                .build(),
        ),
        pure_equity_cache: Arc::new(
            Cache::builder()
                .time_to_live(CACHE_TTL)
                .max_capacity(CACHE_MAX_CAPACITY)
                .build(),
        ),
        nuts_cache: Arc::new(
            Cache::builder()
                .time_to_live(CACHE_TTL)
                .max_capacity(CACHE_MAX_CAPACITY)
                .build(),
        ),
        seven_card_tables: Arc::clone(&seven_card_tables),
    };

    let origins = allowed_origins()?;
    info!(
        "CORS allow-list: {} (plus any http://localhost or http://127.0.0.1 port)",
        origins
            .iter()
            .filter_map(|o| o.to_str().ok())
            .collect::<Vec<_>>()
            .join(", ")
    );

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::predicate(move |origin, _| {
            origins.iter().any(|allowed| allowed == origin) || is_localhost_origin(origin)
        }))
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE]);

    let app = build_router(app_state).layer(cors);

    info!("Listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}