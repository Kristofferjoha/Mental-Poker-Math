use axum::{Router, routing::{get, post}};
use tower_http::cors::{Any, CorsLayer};
use std::{collections::HashMap, net::SocketAddr, sync::{Arc, Mutex}};
use std::fs::read_to_string;
use tracing::info;
use poker_eval::eval::seven as seven_eval;


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

    println!("Building 7-card lookup tables...");
    let seven_card_tables = Arc::new(seven_eval::build_tables(false));
    println!("Tables built successfully.");


    // Shared application state for the Axum server.
    // This struct holds all the data that multiple handlers might need to access concurrently.
    //
    // `pot_eq_store`, `pure_eq_store`, `pure_pot_odds_store`: caches for previously computed
    // results of poker equity and pot odds calculations. These are wrapped in `Arc<Mutex<...>>`
    // so multiple async handlers can safely share and mutate them across threads.
    //
    // `preflop_equity_data`: the preloaded JSON data containing equity values for all possible
    //  preflop hand matchups. Wrapped in `Arc` because it is read-only and can be shared
    //  across threads without locking.
    let app_state = AppState {
        pot_equity_cache: Arc::new(Mutex::new(HashMap::new())),
        pure_equity_cache: Arc::new(Mutex::new(HashMap::new())),
        pure_pot_odds_cache: Arc::new(Mutex::new(HashMap::new())),
        preflop_equity_data: Arc::new(preflop_equity_data),
        seven_card_tables: Arc::clone(&seven_card_tables),
    };

    // Configure CORS (allow all origins, methods, and headers)
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);

    // Build Axum router with API routes
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
