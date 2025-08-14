mod poker_logic;
mod app_state; 

use app_state::AppState;
use poker_logic::api;

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

    let app_state = AppState {
        pot_eq_store: Arc::new(Mutex::new(HashMap::new())),
        pure_eq_store: Arc::new(Mutex::new(HashMap::new())),
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
        .with_state(app_state)
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app)
        .await
        .unwrap_or_else(|err| tracing::error!("Server error: {}", err));
}
