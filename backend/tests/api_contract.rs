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

fn cache<V: Clone + Send + Sync + 'static>() -> Arc<Cache<Uuid, V>> {
    Arc::new(
        Cache::builder()
            .time_to_live(Duration::from_secs(60))
            .max_capacity(100)
            .build()
    )
}

fn state() -> AppState {
    AppState {
        pot_equity_cache: cache(),
        pure_equity_cache: cache(),
        nuts_cache: cache(),
        koth_cache: cache(),
        seven_card_tables: build_tables(false)
    }
}

async fn get(uri: &str) -> (StatusCode, serde_json::Value) {
    let response = build_router(state())
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null)
    )
}

#[tokio::test]
async fn defaults_to_heads_up() {
    let (status, body) = get("/api/pure-equity-get-problem?streets=flop").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["num_players"], 2);
    assert_eq!(body["hands"].as_array().unwrap().len(), 2);
    assert!(body.get("player_hand").is_none());
    assert!(body.get("opponent_hand").is_none());
}

#[tokio::test]
async fn every_seat_count_deals_the_right_number_of_hands() {
    for seats in 2..=6 {
        for endpoint in ["pure-equity", "pot-equity"] {
            let (status, body) =
                get(&format!("/api/{endpoint}-get-problem?numPlayers={seats}")).await;
            assert_eq!(status, StatusCode::OK, "{endpoint} with {seats} seats");
            assert_eq!(body["num_players"], seats);

            let hands = body["hands"].as_array().unwrap();
            assert_eq!(hands.len(), seats);
            for hand in hands {
                assert_eq!(hand.as_array().unwrap().len(), 2, "two cards per seat");
            }
        }
    }
}

#[tokio::test]
async fn six_way_deals_distinct_cards() {
    let (_, body) = get("/api/pure-equity-get-problem?streets=river&numPlayers=6").await;
    let mut seen = std::collections::HashSet::new();
    for hand in body["hands"].as_array().unwrap() {
        for card in hand.as_array().unwrap() {
            assert!(seen.insert(card.to_string()), "duplicate card {card}");
        }
    }
    for card in body["board"].as_array().unwrap() {
        assert!(seen.insert(card.to_string()), "duplicate card {card}");
    }
    assert_eq!(seen.len(), 6 * 2 + 5);
}

#[tokio::test]
async fn bad_seat_counts_are_rejected() {
    for bad in ["1", "7", "0", "abc", "-2", "2.5"] {
        let (status, body) =
            get(&format!("/api/pure-equity-get-problem?numPlayers={bad}")).await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "numPlayers={bad}");
        assert_eq!(body["error"], "invalid_parameter");
    }
}

#[tokio::test]
async fn street_and_seat_count_are_independent() {
    let (_, preflop) = get("/api/pure-equity-get-problem?streets=pre-flop&numPlayers=5").await;
    assert_eq!(preflop["board"].as_array().unwrap().len(), 0);
    assert_eq!(preflop["num_players"], 5);

    let (_, river) = get("/api/pure-equity-get-problem?streets=river&numPlayers=3").await;
    assert_eq!(river["board"].as_array().unwrap().len(), 5);
    assert_eq!(river["num_players"], 3);
}

#[tokio::test]
async fn pot_odds_never_poses_a_river() {
    for _ in 0..12 {
        let (status, body) = get("/api/pot-equity-get-problem?numPlayers=2").await;
        assert_eq!(status, StatusCode::OK);
        let board = body["board"].as_array().unwrap().len();
        assert!(board < 5, "pot odds dealt a complete board: {board} cards");
    }
}

#[tokio::test]
async fn unusable_parameters_are_refused() {
    for uri in [
        "/api/pure-equity-get-problem?streets=banana",
        "/api/pure-equity-get-problem?streets=flop,banana",
        "/api/pure-equity-get-problem?tolerance=banana",
        "/api/pure-equity-get-problem?tolerance=0",
        "/api/pure-equity-get-problem?tolerance=90",
        "/api/pot-equity-get-problem?streets=river",
        "/api/king-of-the-hill-get-problem?streets=river"
    ] {
        let (status, body) = get(uri).await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "{uri} was accepted");
        assert_eq!(body["error"], "invalid_parameter", "{uri}");
    }
}

#[tokio::test]
async fn offered_streets_are_accepted() {
    for (uri, cards) in [
        ("/api/pure-equity-get-problem?streets=river", 5),
        ("/api/pure-equity-get-problem?streets=flop,flop", 3),
        ("/api/pure-equity-get-problem?streets=turn", 4)
    ] {
        let (status, body) = get(uri).await;
        assert_eq!(status, StatusCode::OK, "{uri} was refused");
        assert_eq!(body["board"].as_array().unwrap().len(), cards, "{uri}");
    }
}

#[tokio::test]
async fn nuts_accepts_an_answer_with_no_selection() {
    let app = build_router(state());

    let generated = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/whats-the-nuts-get-problem")
                .body(Body::empty())
                .unwrap()
        )
        .await
        .unwrap();
    let bytes = generated.into_body().collect().await.unwrap().to_bytes();
    let problem: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let id = problem["problem_id"].as_str().expect("problem_id");

    let answered = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/whats-the-nuts-check-answer")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "problemId": id,
                        "selectedIndex": serde_json::Value::Null
                    })
                    .to_string()
                ))
                .unwrap()
        )
        .await
        .unwrap();

    assert_eq!(answered.status(), StatusCode::OK);
    let bytes = answered.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["correct"], false);
    assert!(body["correctHand"].as_array().is_some_and(|h| h.len() == 2));
}

#[tokio::test]
async fn health_reports_the_running_build() {
    let (status, body) = get("/api/health").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["status"], "ok");
    assert_eq!(body["version"], env!("CARGO_PKG_VERSION"));
}
