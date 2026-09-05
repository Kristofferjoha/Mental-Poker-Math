use axum::{
    extract::{Query, State},
    Json
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::poker_core::card::Card;
use crate::problems::whats_the_nuts::{generate, Difficulty};
use crate::utils::api::ApiError;
use crate::utils::app_state::AppState;

#[derive(Serialize)]
pub struct NutsProblemResponse {
    pub problem_id: Uuid,
    pub board: Vec<Card>,
    pub candidates: Vec<Vec<Card>>,
    pub difficulty: &'static str
}

#[derive(Deserialize)]
pub struct NutsAnswerRequest {
    #[serde(rename = "problemId")]
    pub problem_id: Uuid,
    #[serde(rename = "selectedIndex")]
    pub selected_index: Option<usize>
}

#[derive(Serialize)]
pub struct NutsAnswerResponse {
    pub correct: bool,
    #[serde(rename = "correctIndex")]
    pub correct_index: usize,
    #[serde(rename = "correctHand")]
    pub correct_hand: Vec<Card>
}

pub async fn generate_nuts_problem(
    State(app_state): State<AppState>,
    Query(params): Query<HashMap<String, String>>
) -> Result<Json<NutsProblemResponse>, ApiError> {
    let difficulty = match params.get("difficulty") {
        None => Difficulty::Medium,
        Some(raw) => Difficulty::parse(raw).ok_or(ApiError::InvalidParameter {
            name: "difficulty",
            detail: format!("expected easy, medium or hard, got {raw:?}")
        })?
    };

    let problem = generate(difficulty, &app_state.seven_card_tables);
    let problem_id = Uuid::new_v4();
    app_state.nuts_cache.insert(problem_id, problem.clone());

    Ok(Json(NutsProblemResponse {
        problem_id,
        board: problem.board,
        candidates: problem.candidates,
        difficulty: difficulty.as_str()
    }))
}

pub async fn check_nuts_answer(
    State(app_state): State<AppState>,
    Json(payload): Json<NutsAnswerRequest>
) -> Result<Json<NutsAnswerResponse>, ApiError> {
    let problem = app_state
        .nuts_cache
        .get(&payload.problem_id)
        .ok_or(ApiError::ProblemGone(payload.problem_id))?;

    app_state.nuts_cache.invalidate(&payload.problem_id);

    Ok(Json(NutsAnswerResponse {
        correct: payload.selected_index == Some(problem.correct_index),
        correct_index: problem.correct_index,
        correct_hand: problem.candidates[problem.correct_index].clone()
    }))
}
