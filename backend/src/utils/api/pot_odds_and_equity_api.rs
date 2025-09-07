use axum::{extract::{Query, State}, Json};
use serde::{Deserialize, Serialize};
use tracing::{info, debug, warn};
use uuid::Uuid;
use std::collections::HashMap;

use crate::poker_core::card::Card;
use crate::problems::pot_equity::generate;
use crate::utils::app_state::AppState;

/// Response returned when a new Pot Equity problem is generated.
#[derive(Serialize)]
pub struct PotEquityProblemResponse {
    pub problem_id: Uuid,
    pub player_hand: Vec<Card>,
    pub opponent_hand: Vec<Card>,
    pub board: Vec<Card>,
    pub pot_size: u32,
    pub bet_to_call: u32,
}

/// Request sent by the client to check a submitted Pot Equity answer.
#[derive(Deserialize)]
pub struct PotEquityAnswerRequest {
    #[serde(rename = "problemId")]
    pub problem_id: Uuid,
    pub decision: bool, // Player's action: call/fold
}

/// Response returned after checking a submitted Pot Equity answer.
#[derive(Serialize)]
pub struct PotEquityAnswerResponse {
    #[serde(rename = "userGuessIsCorrect")]
    pub user_decision_is_correct: bool,    // Evaluates the user's submitted decision
    #[serde(rename = "expectedDecision")]
    pub expected_decision: bool,           // The correct action (call/fold)
    #[serde(rename = "playerEquity")]
    pub player_equity: f32,
    #[serde(rename = "potOdds")]
    pub pot_odds: f32,
}

/// Generates a new Pot Equity problem.
///
/// `streets` (optional): comma-separated list of allowed streets (default: pre-flop, flop, turn, river)
/// `allowOverbets` (optional, default: true): whether to allow overbets in the problem
pub async fn generate_pot_equity_problem(
    State(app_state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Json<PotEquityProblemResponse> {

    let allowed_streets = params
        .get("streets")
        .map(|s| s.split(',').map(String::from).collect())
        .unwrap_or_else(|| vec![
            "pre-flop".to_string(),
            "flop".to_string(),
            "turn".to_string(),
            "river".to_string(),
        ]);

    let allow_overbets = params
        .get("allowOverbets")
        .map_or(true, |v| v == "true");

    let problem = generate(
        allowed_streets,
        allow_overbets,
        &app_state.preflop_equity_data,
        &app_state.seven_card_tables,
    );

    let problem_id = Uuid::new_v4();
    debug!("Generated POT EQ problem ID: {}, correct decision: {}", problem_id, problem.correct_decision);

    app_state
        .pot_equity_cache
        .lock()
        .expect("mutex poisoned")
        .insert(problem_id, problem.clone());

    Json(PotEquityProblemResponse {
        problem_id,
        player_hand: problem.player_hand,
        opponent_hand: problem.opponent_hand,
        board: problem.board,
        pot_size: problem.pot_size,
        bet_to_call: problem.bet_to_call,
    })
}

/// Checks a submitted Pot Equity answer.
pub async fn check_pot_equity_answer(
    State(app_state): State<AppState>,
    Json(payload): Json<PotEquityAnswerRequest>,
) -> Json<PotEquityAnswerResponse> {
    info!("Checking POT EQ answer for problem ID: {}", payload.problem_id);

    let stored_problem = app_state
        .pot_equity_cache
        .lock()
        .expect("mutex poisoned")
        .remove(&payload.problem_id);

    match stored_problem {
        Some(problem) => {
            let user_decision_is_correct = payload.decision == problem.correct_decision;
            Json(PotEquityAnswerResponse {
                user_decision_is_correct,
                expected_decision: problem.correct_decision,
                player_equity: problem.player_equity,
                pot_odds: problem.pot_odds,
            })
        }
        None => {
            warn!("POT EQ Problem ID not found or expired: {}", payload.problem_id);
            Json(PotEquityAnswerResponse {
                user_decision_is_correct: false,
                expected_decision: false,
                player_equity: 0.0,
                pot_odds: 0.0,
            })
        }
    }
}
