use axum::{extract::{Query, State}, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

use crate::poker_core::card::Card;
use crate::problems::pot_equity::generate;
use crate::utils::api::{params, ApiError};
use crate::utils::app_state::AppState;

#[derive(Serialize)]
pub struct PotEquityProblemResponse {
    pub problem_id: Uuid,
    pub hands: Vec<Vec<Card>>,
    pub board: Vec<Card>,
    pub num_players: usize,
    pub pot_size: u32,
    pub bet_to_call: u32
}

#[derive(Deserialize)]
pub struct PotEquityAnswerRequest {
    #[serde(rename = "problemId")]
    pub problem_id: Uuid,
    pub decision: bool
}

#[derive(Serialize)]
pub struct PotEquityAnswerResponse {
    #[serde(rename = "userGuessIsCorrect")]
    pub user_decision_is_correct: bool,
    #[serde(rename = "expectedDecision")]
    pub expected_decision: bool,
    #[serde(rename = "playerEquity")]
    pub player_equity: f32,
    #[serde(rename = "potOdds")]
    pub pot_odds: f32
}

pub async fn generate_pot_equity_problem(
    State(app_state): State<AppState>,
    Query(params): Query<HashMap<String, String>>
) -> Result<Json<PotEquityProblemResponse>, ApiError> {
    let num_players = params::num_players(&params)?;

    let allowed_streets = params::streets(&params, &["pre-flop", "flop", "turn"])?;

    let allow_overbets = params
        .get("allowOverbets")
        .is_none_or(|v| v == "true");

    let problem = generate(
        allowed_streets,
        num_players,
        allow_overbets,
        &app_state.seven_card_tables
    );

    let problem_id = Uuid::new_v4();

    app_state
        .pot_equity_cache
        .insert(problem_id, problem.clone());

    Ok(Json(PotEquityProblemResponse {
        problem_id,
        hands: problem.hands,
        board: problem.board,
        num_players,
        pot_size: problem.pot_size,
        bet_to_call: problem.bet_to_call
    }))
}

pub async fn check_pot_equity_answer(
    State(app_state): State<AppState>,
    Json(payload): Json<PotEquityAnswerRequest>
) -> Result<Json<PotEquityAnswerResponse>, ApiError> {
    let problem = app_state
        .pot_equity_cache
        .get(&payload.problem_id)
        .ok_or(ApiError::ProblemGone(payload.problem_id))?;

    app_state.pot_equity_cache.invalidate(&payload.problem_id);

    let user_decision_is_correct = payload.decision == problem.correct_decision;
    Ok(Json(PotEquityAnswerResponse {
        user_decision_is_correct,
        expected_decision: problem.correct_decision,
        player_equity: problem.player_equity,
        pot_odds: problem.pot_odds
    }))
}
