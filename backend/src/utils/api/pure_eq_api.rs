use crate::poker_logic::{card::Card, pure_equity_gen};
use crate::utils::app_state::AppState;
use axum::{extract::{State, Query}, Json};
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct PureEqProblemRequest {
    pub problem_id: Uuid,
    pub player_hand: Vec<Card>,
    pub opponent_hand: Vec<Card>,
    pub board: Vec<Card>,
}

#[derive(Deserialize)]
pub struct PureEqCheckAnswerPayload {
    #[serde(rename = "problemId")]
    pub problem_id: Uuid,
    pub guess_value: f32,
}

#[derive(Serialize)]
pub struct PureEqCheckAnswerResponse {
    #[serde(rename = "isCorrect")]
    pub is_correct: bool,
    #[serde(rename = "playerEquity")]
    pub player_equity: f32,
    #[serde(rename = "directionalHint")]
    pub directional_hint: String, // "Higher", "Lower", or "Not-Active"
}

pub async fn get_pure_eq_problem(State(app_state): State<AppState>, Query(params): Query<HashMap<String, String>>) -> Json<PureEqProblemRequest> {
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

    let problem = pure_equity_gen::generate_pure_eq_problem(allowed_streets,
        &app_state.preflop_equity_data,
        tolerance,
        directional_hints_active,
    );
    let problem_id = Uuid::new_v4();

    info!("Generated PURE EQ problem ID: {}", problem_id);
    info!("Problem Equity (hidden from client): {}", problem.player_equity);

    app_state
        .pure_equity_cache
        .lock()
        .unwrap()
        .insert(problem_id, problem.clone());

    Json(PureEqProblemRequest {
        problem_id,
        player_hand: problem.player_hand,
        opponent_hand: problem.opponent_hand,
        board: problem.board,
    })
}


pub async fn pure_eq_check_answer(
    State(app_state): State<AppState>,
    Json(payload): Json<PureEqCheckAnswerPayload>,
) -> Json<PureEqCheckAnswerResponse> {
    info!("Checking PURE EQ answer for problem ID: {}", payload.problem_id);

    let mut cache = app_state.pure_equity_cache.lock().unwrap();

    if let Some(problem) = cache.get(&payload.problem_id) {
        let player_equity = problem.player_equity;
        let directional_hint_active = problem.directional_hint_active;

        let is_correct =
            problem.lower_bound_equity <= payload.guess_value
            && payload.guess_value <= problem.upper_bound_equity;

        // A hint is only relevant if the answer is INCORRECT and hints are enabled.
        let directional_hint = if !is_correct && directional_hint_active {
            if payload.guess_value < player_equity {
                "Higher".to_string()
            } else {
                "Lower".to_string()
            }
        } else {
            "Not-Active".to_string()
        };

        if is_correct {
            info!("PURE EQ answer is correct. Removing problem ID: {}", payload.problem_id);
            cache.remove(&payload.problem_id);
        }

        Json(PureEqCheckAnswerResponse {
            is_correct,
            player_equity,
            directional_hint,
        })
    } else {
        info!("PURE EQ Problem ID not found or expired: {}", payload.problem_id);
        // This default response is correct.
        Json(PureEqCheckAnswerResponse {
            is_correct: false,
            player_equity: 0.0,
            directional_hint: "Not-Active".to_string(),
        })
    }
}