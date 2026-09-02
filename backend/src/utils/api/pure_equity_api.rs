use axum::{extract::{State, Query}, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

use crate::poker_core::card::Card;
use crate::problems::pure_equity::generate;
use crate::calculators::equity_calculator::MAX_PLAYERS;
use crate::utils::api::ApiError;
use crate::utils::app_state::AppState;

#[derive(Serialize)]
pub struct PureEquityProblemResponse {
    pub problem_id: Uuid,
    pub hands: Vec<Vec<Card>>,
    pub board: Vec<Card>,
    pub num_players: usize,
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

fn parse_num_players(params: &HashMap<String, String>) -> Result<usize, ApiError> {
    let Some(raw) = params.get("numPlayers") else {
        return Ok(2);
    };
    let parsed: usize = raw.parse().map_err(|_| ApiError::InvalidParameter {
        name: "numPlayers",
        detail: format!("expected a whole number, got {raw:?}"),
    })?;
    if !(2..=MAX_PLAYERS).contains(&parsed) {
        return Err(ApiError::InvalidParameter {
            name: "numPlayers",
            detail: format!("must be between 2 and {MAX_PLAYERS}, got {parsed}"),
        });
    }
    Ok(parsed)
}

/// Generates a new Pure Equity problem.
///
/// `streets` (optional): comma-separated list of allowed streets (default: pre-flop, flop, turn, river)
/// `tolerance` (optional): acceptable error margin for the answer (default: 5.0)
/// `directionalHints` (optional): enable hints if answer is wrong (default: false)
/// 
pub async fn generate_pure_equity_problem(
    State(app_state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<PureEquityProblemResponse>, ApiError> {
    let num_players = parse_num_players(&params)?;

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
        num_players,
        tolerance,
        directional_hints_active,
        &app_state.seven_card_tables,
    );

    let problem_id = Uuid::new_v4();

    // Stores the generated problem in the cache for later answer checking.
    app_state
        .pure_equity_cache
        .insert(problem_id, problem.clone());

    Ok(Json(PureEquityProblemResponse {
        problem_id,
        hands: problem.hands,
        board: problem.board,
        num_players,
    }))
}

/// Checks a submitted Pure Equity answer.
///
/// Returns `410 Gone` when the problem id is unknown, never a graded answer.
pub async fn check_pure_equity_answer(
    State(app_state): State<AppState>,
    Json(payload): Json<PureEquityAnswerRequest>,
) -> Result<Json<PureEquityAnswerResponse>, ApiError> {
    let problem = app_state
        .pure_equity_cache
        .get(&payload.problem_id)
        .ok_or(ApiError::ProblemGone(payload.problem_id))?;

    let player_equity = problem.player_equity;
    let directional_hint_active = problem.directional_hint_active;

    let user_guess_is_correct = problem.lower_bound_equity <= payload.guess_value
        && payload.guess_value <= problem.upper_bound_equity;

    // Provide directional hint if the guess is incorrect and hints are enabled.
    let directional_hint = if !user_guess_is_correct && directional_hint_active {
        if payload.guess_value < player_equity {
            "Higher"
        } else {
            "Lower"
        }
    } else {
        "Not-Active"
    }
    .to_string();

    if user_guess_is_correct {
        app_state.pure_equity_cache.invalidate(&payload.problem_id);
    }

    Ok(Json(PureEquityAnswerResponse {
        user_guess_is_correct,
        player_equity,
        directional_hint,
    }))
}
