//! A check endpoint must never grade an answer it cannot verify.
//!
//! Before this, an unknown problem id returned `200 OK` with zeroed fields --
//! telling the player they were wrong and that the correct play was to fold. A
//! cache miss happens on expiry, on a retry after the entry was invalidated, and
//! on every server restart, so this is not a rare path.

use std::sync::Arc;
use std::time::Duration;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use moka::sync::Cache;
use poker_eval::eval::seven::build_tables;
use tower::ServiceExt;
use uuid::Uuid;

use backend::program::build_router;
use backend::utils::app_state::AppState;

/// A closure here would monomorphize to a single value type, so this is generic.
fn cache<V: Clone + Send + Sync + 'static>() -> Arc<Cache<Uuid, V>> {
    Arc::new(
        Cache::builder()
            .time_to_live(Duration::from_secs(60))
            .max_capacity(100)
            .build(),
    )
}

fn test_state() -> AppState {
    AppState {
        pot_equity_cache: cache(),
        pure_equity_cache: cache(),
        nuts_cache: cache(),
        seven_card_tables: build_tables(false),
    }
}

async fn post(path: &str, body: serde_json::Value) -> (StatusCode, serde_json::Value) {
    let response = build_router(test_state())
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(path)
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let json = serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null);
    (status, json)
}

#[tokio::test]
async fn unknown_pure_equity_problem_is_gone_not_graded() {
    let id = Uuid::new_v4();
    let (status, body) = post(
        "/api/pure-equity-check-answer",
        serde_json::json!({ "problemId": id, "guess_value": 50.0 }),
    )
    .await;

    assert_eq!(status, StatusCode::GONE);
    assert_eq!(body["error"], "problem_gone");
    // the old behaviour: 200 with playerEquity 0.0 and userGuessIsCorrect false
    assert!(body.get("playerEquity").is_none());
    assert!(body.get("userGuessIsCorrect").is_none());
}

#[tokio::test]
async fn unknown_pot_equity_problem_does_not_invent_a_fold() {
    let id = Uuid::new_v4();
    let (status, body) = post(
        "/api/pot-equity-check-answer",
        serde_json::json!({ "problemId": id, "decision": true }),
    )
    .await;

    assert_eq!(status, StatusCode::GONE);
    assert_eq!(body["error"], "problem_gone");
    // `expected_decision: false` used to read as a real answer -- "you should have folded"
    assert!(body.get("expected_decision").is_none());
    assert!(body.get("player_equity").is_none());
}

/// The happy path still works: generate a real problem, then answer it.
#[tokio::test]
async fn a_freshly_generated_problem_can_still_be_answered() {
    let state = test_state();
    let app = build_router(state.clone());

    let generated = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/pure-equity-get-problem?streets=river")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(generated.status(), StatusCode::OK);

    let bytes = generated.into_body().collect().await.unwrap().to_bytes();
    let problem: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let id = problem["problem_id"].as_str().expect("problem_id");

    let answered = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/pure-equity-check-answer")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({ "problemId": id, "guess_value": 50.0 }).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(answered.status(), StatusCode::OK);
    let bytes = answered.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert!(body.get("playerEquity").is_some());
}
