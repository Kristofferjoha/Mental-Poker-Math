use crate::poker_logic::{card::Card, pure_equity_gen};
use crate::AppState;
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
    let problem = pure_equity_gen::generate_pure_eq_problem(allowed_streets,
        &app_state.preflop_equity_data,
    );
    let problem_id = Uuid::new_v4();

    info!("Generated PURE EQ problem ID: {}", problem_id);
    info!("Problem Equity (hidden from client): {}", problem.player_equity);

    app_state
        .pure_eq_store
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

    let mut store = app_state.pure_eq_store.lock().unwrap();

    let stored_problem_ref = store.get(&payload.problem_id);

    if let Some(problem_ref) = stored_problem_ref {
        let problem_clone = problem_ref.clone();

        let is_correct =
            problem_clone.lower_bound_equity <= payload.guess_value
                && payload.guess_value <= problem_clone.upper_bound_equity;

        if is_correct {
            info!("PURE EQ answer is correct. Removing problem ID: {}", payload.problem_id);
            store.remove(&payload.problem_id);
        }

        Json(PureEqCheckAnswerResponse {
            is_correct,
            player_equity: problem_clone.player_equity,
        })
    } else {
        info!("PURE EQ Problem ID not found or expired: {}", payload.problem_id);
        Json(PureEqCheckAnswerResponse {
            is_correct: false,
            player_equity: 0.0,
        })
    }
}