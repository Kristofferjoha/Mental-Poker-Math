// $env:RUST_LOG = "info"   
mod poker_logic;
mod app_state; 

use app_state::AppState;
use poker_logic::api;
use std::fs;

use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let preflop_data_string = fs::read_to_string("preflop_equity.json").unwrap();
    let preflop_equity_data: Vec<PreflopEquity> = serde_json::from_str(&preflop_data_string).unwrap();

    let app_state = AppState {
        pot_eq_store: Arc::new(Mutex::new(HashMap::new())),
        pure_eq_store: Arc::new(Mutex::new(HashMap::new())),
        pure_pot_odds_store: Arc::new(Mutex::new(HashMap::new())),
        preflop_equity_data: Arc::new(preflop_equity_data),

    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/get-problem", get(api::get_new_problem))
        .route("/api/check-answer", post(api::check_answer))
        .route("/api/pure-eq-get-problem", get(api::get_pure_eq_problem))
        .route("/api/pure-eq-check-answer", post(api::pure_eq_check_answer))
        .route("/api/pure-pot-odds-get-problem", get(api::get_pure_pot_odds_problem))
        .route("/api/pure-pot-odds-check-answer", post(api::pure_pot_odds_check_answer))
        .with_state(app_state)
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app)
        .await
        .unwrap_or_else(|err| tracing::error!("Server error: {}", err));
}
