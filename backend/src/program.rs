use axum::{Router, routing::{get, post}};
use tower_http::cors::{Any, CorsLayer};
use std::{net::SocketAddr, sync::{Arc}};
use std::fs::read_to_string;
use tracing::info;
use poker_eval::eval::seven as seven_eval;
use std::time::Duration;
use moka::sync::Cache;


use crate::preflop_data::preflop_lookup::PreflopEquity;
use crate::utils::{api, app_state::AppState};

/// Main entrypoint for the Axum application.
/// Sets up state, routes, and starts the HTTP server.
/// 
pub async fn run() -> anyhow::Result<()> {
    // Load preflop equity data from JSON
    info!("Loading preflop equity data...");
    let preflop_data_string = read_to_string("src/preflop_data/preflop_equity.json")?;
    let preflop_equity_data: Vec<PreflopEquity> = serde_json::from_str(&preflop_data_string)?;
    info!("Loaded {} entries", preflop_equity_data.len());

    info!("Building 7-card lookup tables...");
    let seven_card_tables = Arc::new(seven_eval::build_tables(false));
    info!("Tables built successfully.");


    // Shared application state for the Axum server.
    // This struct bundles together all resources that multiple handlers may need to access.
    //
    // - `pot_equity_cache`, `pure_equity_cache`, `pure_pot_odds_cache`: 
    //   in-memory caches (with TTL and capacity limits) for storing results of
    //   poker equity and pot odds calculations. These caches are thread-safe 
    //   and can be accessed concurrently without explicit locking.
    //
    // - `preflop_equity_data`: preloaded JSON data containing equity values for 
    //   all possible preflop hand matchups. Wrapped in `Arc` since it is read-only 
    //   and can be shared efficiently across threads.
    //
    // - `seven_card_tables`: precomputed lookup tables for evaluating 7-card 
    //   poker hands. Also shared across threads via `Arc`.

    const CACHE_TTL: Duration = Duration::from_secs(300); 
    const CACHE_MAX_CAPACITY: u64 = 10_000;

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

    // Configure CORS (allows all origins, methods, and headers)
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);

    // Axum router with API routes
    let app = Router::new()
        .route("/api/pot-equity-get-problem", get(api::generate_pot_equity_problem)) // Corrected
        .route("/api/pot-equity-check-answer", post(api::check_pot_equity_answer)) // Corrected
        .route("/api/pure-equity-get-problem", get(api::generate_pure_equity_problem))
        .route("/api/pure-equity-check-answer", post(api::check_pure_equity_answer))
        .route("/api/pure-pot-odds-get-problem", get(api::generate_pure_pot_odds_problem))
        .route("/api/pure-pot-odds-check-answer", post(api::check_pure_pot_odds_answer))
        .with_state(app_state)
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
