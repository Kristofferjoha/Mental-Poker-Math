use crate::poker_logic::{pure_pot_odds_gen};
use crate::utils::app_state::AppState;
use axum::{extract::{Query,State}, Json,};
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct PurePotOddsProblemRequest {
    pub problem_id: Uuid,
    pub pot_size: u32,
    pub bet_to_call: u32,
    pub equity: f64,
}

#[derive(Deserialize)]
pub struct PurePotOddsCheckAnswerPayload {
    #[serde(rename = "problemId")]
    pub problem_id: Uuid,
    pub decision: bool,
}

#[derive(Serialize)]
pub struct PurePotOddsCheckAnswerResponse {
    #[serde(rename = "isCorrect")]
    pub is_correct: bool,
    #[serde(rename = "correctDecision")]
    pub correct_decision: bool,
    #[serde(rename = "potOdds")]
    pub pot_odds: f64,
}

pub async fn get_pure_pot_odds_problem(State(app_state): State<AppState>, Query(params): Query<HashMap<String, String>>) -> Json<PurePotOddsProblemRequest> {
    let allow_overbets = params.get("allowOverbets").map(|v| v == "true").unwrap_or(true);

    let problem = pure_pot_odds_gen::generate_pure_pot_odds_problem(allow_overbets);
    let problem_id = Uuid::new_v4();

    info!("Generated Pure Pot Odds problem ID: {}", problem_id);
    info!("Correct decision (hidden from client): {}", problem.correct_decision);
    info!("pot size: {}, bet to call: {}", problem.pot_size, problem.bet_to_call);

    app_state
        .pure_pot_odds_cache
        .lock()
        .unwrap()
        .insert(problem_id, problem.clone());

    Json(PurePotOddsProblemRequest {
        problem_id,
        pot_size: problem.pot_size,
        bet_to_call: problem.bet_to_call,
        equity: problem.equity,
    })
}

pub async fn pure_pot_odds_check_answer(
    State(app_state): State<AppState>,
    Json(payload): Json<PurePotOddsCheckAnswerPayload>,
) -> Json<PurePotOddsCheckAnswerResponse> {
    info!("Checking POT EQ answer for problem ID: {}", payload.problem_id);
    let stored_problem = app_state
        .pure_pot_odds_cache
        .lock()
        .unwrap()
        .remove(&payload.problem_id);

    if let Some(problem) = stored_problem {
        let is_correct = payload.decision == problem.correct_decision;

        Json(PurePotOddsCheckAnswerResponse {
            is_correct,
            correct_decision: problem.correct_decision,
            pot_odds: problem.pot_odds,
        })
    } else {
        info!("POT EQ Problem ID not found or expired: {}", payload.problem_id);
        Json(PurePotOddsCheckAnswerResponse {
            is_correct: false,
            correct_decision: false,
            pot_odds: 0.0,
        })
    }
}