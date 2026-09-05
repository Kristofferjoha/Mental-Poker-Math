use axum::{extract::{State, Query}, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

use crate::poker_core::card::Card;
use crate::problems::pure_equity::generate;
use crate::utils::api::{params, ApiError};
use crate::utils::app_state::AppState;

#[derive(Serialize)]
pub struct PureEquityProblemResponse {
    pub problem_id: Uuid,
    pub hands: Vec<Vec<Card>>,
    pub board: Vec<Card>,
    pub num_players: usize,
    pub player_equity: f32,
    pub lower_bound: f32,
    pub upper_bound: f32
}

#[derive(Deserialize)]
pub struct PureEquityAnswerRequest {
    #[serde(rename = "problemId")]
    pub problem_id: Uuid,
    pub guess_value: f32
}

#[derive(Serialize)]
pub struct PureEquityAnswerResponse {
    #[serde(rename = "userGuessIsCorrect")]
    pub user_guess_is_correct: bool,
    #[serde(rename = "playerEquity")]
    pub player_equity: f32
}

fn default_tolerance(num_players: usize) -> f32 {
    10.0 / num_players as f32
}

pub async fn generate_pure_equity_problem(
    State(app_state): State<AppState>,
    Query(params): Query<HashMap<String, String>>
) -> Result<Json<PureEquityProblemResponse>, ApiError> {
    let num_players = params::num_players(&params)?;

    let allowed_streets = params::streets(&params, &["pre-flop", "flop", "turn", "river"])?;
    let tolerance = params::tolerance(&params, default_tolerance(num_players))?;

    let problem = generate(
        allowed_streets,
        num_players,
        tolerance,
        &app_state.seven_card_tables
    );

    let problem_id = Uuid::new_v4();

    app_state
        .pure_equity_cache
        .insert(problem_id, problem.clone());

    Ok(Json(PureEquityProblemResponse {
        problem_id,
        hands: problem.hands,
        board: problem.board,
        num_players,
        player_equity: problem.player_equity,
        lower_bound: problem.lower_bound_equity,
        upper_bound: problem.upper_bound_equity
    }))
}

pub async fn check_pure_equity_answer(
    State(app_state): State<AppState>,
    Json(payload): Json<PureEquityAnswerRequest>
) -> Result<Json<PureEquityAnswerResponse>, ApiError> {
    let problem = app_state
        .pure_equity_cache
        .get(&payload.problem_id)
        .ok_or(ApiError::ProblemGone(payload.problem_id))?;

    let user_guess_is_correct = problem.lower_bound_equity <= payload.guess_value
        && payload.guess_value <= problem.upper_bound_equity;

    app_state.pure_equity_cache.invalidate(&payload.problem_id);

    Ok(Json(PureEquityAnswerResponse {
        user_guess_is_correct,
        player_equity: problem.player_equity
    }))
}
