use axum::Json;
use serde::Serialize;
use crate::poker_logic::{card::Card, problem_generator};
use tracing::info;


#[derive(Serialize, Debug)]
pub struct ApiProblemResponse {
    player_hand: Vec<Card>,
    opponent_hand: Vec<Card>,
    board: Vec<Card>,
    stage: u8,
    pot_size: u32,
    bet_to_call: u32,
    player_equity: f32,
    pot_odds: f32,
    correct_decision: bool,
}

pub async fn get_new_problem() -> Json<ApiProblemResponse> {
    let problem = problem_generator::generate_pot_eq_problem();

    let api_response = ApiProblemResponse {
        player_hand: problem.player_hand,
        opponent_hand: problem.opponent_hand,
        board: problem.board,
        stage: problem.stage,
        pot_size: problem.pot_size,
        bet_to_call: problem.bet_to_call,
        player_equity: (problem.player_equity * 100.0).round() / 100.0,
        pot_odds: (problem.pot_odds * 100.0).round() / 100.0, 
        correct_decision: problem.correct_decision,
    };

    // info!("Generated new problem: {:?}", api_response);

    Json(api_response)
}