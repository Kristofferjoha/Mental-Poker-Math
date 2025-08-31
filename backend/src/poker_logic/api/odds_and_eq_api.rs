use crate::poker_logic::{card::Card, problem_generator};
use crate::AppState;
use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

#[derive(Serialize)]
pub struct ProblemRequest {
    pub problem_id: Uuid,
    pub player_hand: Vec<Card>,
    pub opponent_hand: Vec<Card>,
    pub board: Vec<Card>,
    pub pot_size: u32,
    pub bet_to_call: u32,
}

#[derive(Deserialize)]
pub struct CheckAnswerPayload {
    #[serde(rename = "problemId")]
    pub problem_id: Uuid,
    pub decision: bool,
}

#[derive(Serialize)]
pub struct CheckAnswerResponse {
    #[serde(rename = "isCorrect")]
    pub is_correct: bool,
    #[serde(rename = "correctDecision")]
    pub correct_decision: bool,
    #[serde(rename = "playerEquity")]
    pub player_equity: f32,
    #[serde(rename = "potOdds")]
    pub pot_odds: f32,
}

pub async fn get_new_problem(State(app_state): State<AppState>) -> Json<ProblemRequest> {
    let problem = problem_generator::generate_pot_eq_problem();
    let problem_id = Uuid::new_v4();

    info!("Generated POT EQ problem ID: {}", problem_id);
    info!("Correct decision (hidden from client): {}", problem.correct_decision);

    app_state
        .pot_eq_store
        .lock()
        .unwrap()
        .insert(problem_id, problem.clone());

    Json(ProblemRequest {
        problem_id,
        player_hand: problem.player_hand,
        opponent_hand: problem.opponent_hand,
        board: problem.board,
        pot_size: problem.pot_size,
        bet_to_call: problem.bet_to_call,
    })
}

pub async fn check_answer(
    State(app_state): State<AppState>,
    Json(payload): Json<CheckAnswerPayload>,
) -> Json<CheckAnswerResponse> {
    info!("Checking POT EQ answer for problem ID: {}", payload.problem_id);
    let stored_problem = app_state
        .pot_eq_store
        .lock()
        .unwrap()
        .remove(&payload.problem_id);

    if let Some(problem) = stored_problem {
        let is_correct = payload.decision == problem.correct_decision;

        Json(CheckAnswerResponse {
            is_correct,
            correct_decision: problem.correct_decision,
            player_equity: problem.player_equity,
            pot_odds: problem.pot_odds,
        })
    } else {
        info!("POT EQ Problem ID not found or expired: {}", payload.problem_id);
        Json(CheckAnswerResponse {
            is_correct: false,
            correct_decision: false,
            player_equity: 0.0,
            pot_odds: 0.0,
        })
    }
}