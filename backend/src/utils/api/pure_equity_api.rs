use axum::{extract::{State, Query}, Json};
use serde::{Deserialize, Serialize};
use tracing::{info, debug, warn};
use uuid::Uuid;
use std::collections::HashMap;

use crate::poker_core::card::Card;
use crate::problems::pure_equity::generate;
use crate::utils::app_state::AppState;

/// Response returned when a new Pure Equity problem is generated.
#[derive(Serialize)]
pub struct PureEquityProblemResponse {
    pub problem_id: Uuid,
    pub player_hand: Vec<Card>,
    pub opponent_hand: Vec<Card>,
    pub board: Vec<Card>,
}

/// Request sent by the client to check a submitted Pure Equity answer.
#[derive(Deserialize)]
pub struct PureEquityAnswerRequest {
    #[serde(rename = "problemId")]
    pub problem_id: Uuid,
    pub guess_value: f32, // The player's estimated equity
}

/// Response returned after checking a submitted Pure Equity answer.
#[derive(Serialize)]
pub struct PureEquityAnswerResponse {
    #[serde(rename = "userGuessIsCorrect")]
    pub user_guess_is_correct: bool,           // Evaluates the user's submitted guess
    #[serde(rename = "playerEquity")]
    pub player_equity: f32,                    // The actual equity of the player hand
    #[serde(rename = "directionalHint")]
    pub directional_hint: String,              // "Higher", "Lower", or "Not-Active"
}

/// Generates a new Pure Equity problem.
///
/// `streets` (optional): comma-separated list of allowed streets (default: pre-flop, flop, turn, river)
/// `tolerance` (optional): acceptable error margin for the answer (default: 5.0)
/// `directionalHints` (optional): enable hints if answer is wrong (default: false)
pub async fn generate_pure_equity_problem(
    State(app_state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Json<PureEquityProblemResponse> {
    let allowed_streets = params
        .get("streets")
        .map(|s| s.split(',').map(String::from).collect())
        .unwrap_or_else(|| vec![
            "pre-flop".to_string(),
            "flop".to_string(),
            "turn".to_string(),
            "river".to_string(),
        ]);
    
    let tolerance: f32 = params
        .get("tolerance")
        .and_then(|t| t.parse().ok())
        .unwrap_or(5.0);

    let directional_hints_active: bool = params
        .get("directionalHints")
        .map(|v| v == "true" || v == "1")
        .unwrap_or(false);

    let problem = generate(
        allowed_streets,
        &app_state.preflop_equity_data,
        tolerance,
        directional_hints_active,
        &app_state.seven_card_tables,
    );

    let problem_id = Uuid::new_v4();
    debug!("Generated PURE EQ problem ID: {}, player equity: {}", problem_id, problem.player_equity);

    app_state
        .pure_equity_cache
        .lock()
        .expect("mutex poisoned")
        .insert(problem_id, problem.clone());

    Json(PureEquityProblemResponse {
        problem_id,
        player_hand: problem.player_hand,
        opponent_hand: problem.opponent_hand,
        board: problem.board,
    })
}

/// Checks a submitted Pure Equity answer.
pub async fn check_pure_equity_answer(
    State(app_state): State<AppState>,
    Json(payload): Json<PureEquityAnswerRequest>,
) -> Json<PureEquityAnswerResponse> {
    info!("Checking PURE EQ answer for problem ID: {}", payload.problem_id);

    let mut cache = app_state.pure_equity_cache.lock().expect("mutex poisoned");

    if let Some(problem) = cache.get(&payload.problem_id) {
        let player_equity = problem.player_equity;
        let directional_hint_active = problem.directional_hint_active;

        let user_guess_is_correct =
            problem.lower_bound_equity <= payload.guess_value &&
            payload.guess_value <= problem.upper_bound_equity;

        let directional_hint = if !user_guess_is_correct && directional_hint_active {
            if payload.guess_value < player_equity { "Higher" } else { "Lower" }
        } else {
            "Not-Active"
        }.to_string();

        if user_guess_is_correct {
            info!("PURE EQ answer is correct. Removing problem ID: {}", payload.problem_id);
            cache.remove(&payload.problem_id);
        }

        Json(PureEquityAnswerResponse {
            user_guess_is_correct,
            player_equity,
            directional_hint,
        })
    } else {
        warn!("PURE EQ Problem ID not found or expired: {}", payload.problem_id);
        Json(PureEquityAnswerResponse {
            user_guess_is_correct: false,
            player_equity: 0.0,
            directional_hint: "Not-Active".to_string(),
        })
    }
}
