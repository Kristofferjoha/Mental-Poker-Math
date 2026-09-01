use axum::{Router, routing::{get, post}};
use tower_http::cors::CorsLayer;
use std::{net::SocketAddr, sync::Arc, fs::read_to_string, time::Duration, env};
use tracing::{info, warn};
use poker_eval::eval::seven as seven_eval;
use moka::sync::Cache;
use axum::http::{header, HeaderValue, Method};

use crate::preflop_data::preflop_lookup::PreflopEquity;
use crate::utils::{api, app_state::AppState};

/// Origins accepted when `CORS_ALLOWED_ORIGINS` is unset: production, plus the
/// local dev servers. Browsers treat `localhost` and `127.0.0.1` as *different*
/// origins, so both spellings are listed. 5173 is Vite/SvelteKit; 3000 is Next.js.
const DEFAULT_ALLOWED_ORIGINS: &[&str] = &[
    "https://mentalpokermath.com",
    "https://www.mentalpokermath.com",
    "http://localhost:5173",
    "http://127.0.0.1:5173",
    "http://localhost:3000",
    "http://127.0.0.1:3000",
];

/// Checks that an allow-list entry is a usable origin before it reaches the CORS
/// layer.
///
/// `HeaderValue` accepts any visible ASCII, so it alone would happily admit
/// `htp://localhost` or a trailing slash. Neither can ever match a browser's
/// `Origin` header, and the resulting failure looks identical to the origin
/// simply not being listed -- so reject them loudly here instead.
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

/// Reads the CORS allow-list from `CORS_ALLOWED_ORIGINS` (comma-separated),
/// falling back to [`DEFAULT_ALLOWED_ORIGINS`].
///
/// A malformed entry is logged and skipped rather than panicking the server on
/// boot. An empty list is a hard error: it would reject every browser origin,
/// which fails silently from the client's side and is painful to diagnose.
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

/// Builds the application router.
///
/// Split out of [`run`] so integration tests can drive the handlers with
/// `tower::ServiceExt::oneshot`, without binding a socket or loading data files.
/// The CORS layer is applied by [`run`] rather than here, since tests do not
/// need it and it is configured from the environment.
pub fn build_router(app_state: AppState) -> Router {
    Router::new()
        .route("/api/pot-equity-get-problem", get(api::generate_pot_equity_problem))
        .route("/api/pot-equity-check-answer", post(api::check_pot_equity_answer))
        .route("/api/pure-equity-get-problem", get(api::generate_pure_equity_problem))
        .route("/api/pure-equity-check-answer", post(api::check_pure_equity_answer))
        .route("/api/pure-pot-odds-get-problem", get(api::generate_pure_pot_odds_problem))
        .route("/api/pure-pot-odds-check-answer", post(api::check_pure_pot_odds_answer))
        .with_state(app_state)
}

/// Main entrypoint for the Axum application.
/// Sets up state, routes, and starts the HTTP server.
pub async fn run() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let host_str = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port_str = env::var("PORT").unwrap_or_else(|_| "8001".to_string());
    let data_path = env::var("PREFLOP_DATA_PATH")
        .unwrap_or_else(|_| "src/preflop_data/preflop_equity.json".to_string());
    
    let addr_str = format!("{}:{}", host_str, port_str);
    let addr: SocketAddr = addr_str.parse().expect("Unable to parse socket address");

    info!("Loading preflop equity data from {}...", data_path);
    let preflop_data_string = read_to_string(data_path)?;
    let preflop_equity_data: Vec<PreflopEquity> = serde_json::from_str(&preflop_data_string)?;
    info!("Loaded {} entries", preflop_equity_data.len());

    info!("Building 7-card lookup tables...");
    let seven_card_tables = Arc::new(seven_eval::build_tables(false));
    info!("Tables built successfully.");

    // Generous on purpose. A player who leaves a tab open should not come back to
    // an expired problem -- and an expired problem is one of the few ways the
    // check endpoints can no longer answer. Entries are a few hundred bytes each,
    // so 50k of them costs roughly 15 MB.
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
        pure_pot_odds_cache: Arc::new(
            Cache::builder()
                .time_to_live(CACHE_TTL)
                .max_capacity(CACHE_MAX_CAPACITY)
                .build(),
        ),
        preflop_equity_data: Arc::new(preflop_equity_data),
        seven_card_tables: Arc::clone(&seven_card_tables),
    };

    let origins = allowed_origins()?;
    info!(
        "CORS allow-list: {}",
        origins
            .iter()
            .filter_map(|o| o.to_str().ok())
            .collect::<Vec<_>>()
            .join(", ")
    );

    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE]);

    let app = build_router(app_state).layer(cors);

    info!("Listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}