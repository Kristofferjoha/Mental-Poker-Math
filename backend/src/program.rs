use axum::{Router, routing::{get, post}};
use tower_http::cors::CorsLayer;
use std::{net::SocketAddr, sync::Arc, fs::read_to_string, time::Duration, env};
use tracing::info;
use poker_eval::eval::seven as seven_eval;
use moka::sync::Cache;
use axum::http::{header, HeaderValue, Method};

use crate::preflop_data::preflop_lookup::PreflopEquity;
use crate::utils::{api, app_state::AppState};

/// Main entrypoint for the Axum application.
/// Sets up state, routes, and starts the HTTP server.
pub async fn run() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let host_str = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port_str = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
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

    let origins = [
    "https://mentalpokermath.com".parse::<HeaderValue>().unwrap(),
    "https://www.mentalpokermath.com".parse::<HeaderValue>().unwrap(),
    ];

    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE]);

    let app = Router::new()
        .route("/api/pot-equity-get-problem", get(api::generate_pot_equity_problem))
        .route("/api/pot-equity-check-answer", post(api::check_pot_equity_answer))
        .route("/api/pure-equity-get-problem", get(api::generate_pure_equity_problem))
        .route("/api/pure-equity-check-answer", post(api::check_pure_equity_answer))
        .route("/api/pure-pot-odds-get-problem", get(api::generate_pure_pot_odds_problem))
        .route("/api/pure-pot-odds-check-answer", post(api::check_pure_pot_odds_answer))
        .with_state(app_state)
        .layer(cors);

    info!("Listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}