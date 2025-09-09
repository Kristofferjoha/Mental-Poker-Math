use axum::{extract::{Query,State}, Json,};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use crate::problems::pure_pot_odds::generate;
use crate::utils::app_state::AppState;

/// Response returned when a new Pure Pot Odds problem is generated.
#[derive(Serialize)]
pub struct PurePotOddsProblemResponse {
    pub problem_id: Uuid,
    pub pot_size: u32,
    pub bet_to_call: u32,
    pub equity: f64,
}

#[derive(Deserialize)]
pub struct PurePotOddsAnswerRequest {
    #[serde(rename = "problemId")]
    pub problem_id: Uuid,
    pub user_decision: bool,
}

#[derive(Serialize)]
pub struct PurePotOddsAnswerResponse {
    #[serde(rename = "userGuessIsCorrect")] // used for score game score
    pub user_decision_is_correct: bool,
    #[serde(rename = "expectedDecision")] // used for potential session reviews, correct decision shown
    pub expected_decision: bool,
    #[serde(rename = "potOdds")]
    pub pot_odds: f64,
}

/// Generates a new Pure Pot Odds problem.
///
/// `allowOverbets` (optional, default `false`): whether to allow overbets in the problem.
///
/// Returns a `PurePotOddsProblemResponse` containing the problem ID, pot size, bet to call, and equity.
/// The generate_pure_pot_odds_problem also stores correct_decision but wont be sent to frontend.
pub async fn generate_pure_pot_odds_problem(
    State(app_state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Json<PurePotOddsProblemResponse> {
    let allow_overbets = params.get("allowOverbets").map(|v| v == "true").unwrap_or(false);


    let problem = generate(allow_overbets);
    let problem_id = Uuid::new_v4();

    app_state
        .pure_pot_odds_cache
        .lock()
        .expect("mutex poisoned")
        .insert(problem_id, problem.clone());

    Json(PurePotOddsProblemResponse {
        problem_id,
        pot_size: problem.pot_size,
        bet_to_call: problem.bet_to_call,
        equity: problem.equity,
    })
}

/// Checks the answer to a previously generated Pure Pot Odds problem.
///
/// Accepts a `PurePotOddsAnswerRequest` with the problem ID and the user's decision.
/// Returns `PurePotOddsAnswerResponse` indicating correctness, the correct decision, and the pot odds.
/// payload corresponds to user answer request data, and problem corresponds to the cached problem (UUID)
/// Checks if user decision matches correct decision in problem
pub async fn check_pure_pot_odds_answer(
    State(app_state): State<AppState>,
    Json(payload): Json<PurePotOddsAnswerRequest>,
) -> Json<PurePotOddsAnswerResponse> {

    let stored_problem = app_state
        .pure_pot_odds_cache
        .lock()
        .expect("mutex poisoned")
        .remove(&payload.problem_id);

    match stored_problem {
        Some(problem) => {
            let user_decision_is_correct = payload.user_decision == problem.correct_decision;
            Json(PurePotOddsAnswerResponse {
                user_decision_is_correct,
                expected_decision: problem.correct_decision,
                pot_odds: problem.pot_odds,
            })
        }
        None => {
            tracing::warn!("POT EQ Problem ID not found or expired: {}", payload.problem_id);
            Json(PurePotOddsAnswerResponse {
                user_decision_is_correct: false,
                expected_decision: false,
                pot_odds: 0.0,
            })
        }
    }
}