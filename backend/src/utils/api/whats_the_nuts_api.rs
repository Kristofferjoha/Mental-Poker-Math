use axum::{
    extract::{Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::poker_core::card::Card;
use crate::problems::whats_the_nuts::{generate, Difficulty};
use crate::utils::api::ApiError;
use crate::utils::app_state::AppState;

/// Response returned when a new What's the Nuts problem is generated.
///
/// `correct_index` is deliberately absent: it is held server-side until the
/// answer comes back.
#[derive(Serialize)]
pub struct NutsProblemResponse {
    pub problem_id: Uuid,
    pub board: Vec<Card>,
    pub time_limit_ms: u32,
    /// A shortlist of plausible two-card holdings, one of which is the nuts.
    pub candidates: Vec<Vec<Card>>,
    pub difficulty: &'static str,
}

#[derive(Deserialize)]
pub struct NutsAnswerRequest {
    #[serde(rename = "problemId")]
    pub problem_id: Uuid,
    #[serde(rename = "selectedIndex")]
    pub selected_index: usize,
    #[serde(rename = "responseTimeMs")]
    pub response_time_ms: u32,
}

#[derive(Serialize)]
pub struct NutsAnswerResponse {
    pub correct: bool,
    #[serde(rename = "correctIndex")]
    pub correct_index: usize,
    #[serde(rename = "correctHand")]
    pub correct_hand: Vec<Card>,
    /// Whether the answer arrived inside `time_limit_ms`. Reported separately
    /// from `correct` so a right-but-slow answer is distinguishable from a wrong
    /// one -- this is a speed drill, and those are different mistakes.
    #[serde(rename = "withinTimeLimit")]
    pub within_time_limit: bool,
}

/// Generates a new What's the Nuts problem.
///
/// `difficulty` (optional): easy | medium | hard (default: medium)
pub async fn generate_nuts_problem(
    State(app_state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<NutsProblemResponse>, ApiError> {
    let difficulty = match params.get("difficulty") {
        None => Difficulty::Medium,
        Some(raw) => Difficulty::parse(raw).ok_or(ApiError::InvalidParameter {
            name: "difficulty",
            detail: format!("expected easy, medium or hard, got {raw:?}"),
        })?,
    };

    let problem = generate(difficulty, &app_state.seven_card_tables);
    let problem_id = Uuid::new_v4();
    app_state.nuts_cache.insert(problem_id, problem.clone());

    Ok(Json(NutsProblemResponse {
        problem_id,
        board: problem.board,
        time_limit_ms: problem.time_limit_ms,
        candidates: problem.candidates,
        difficulty: difficulty.as_str(),
    }))
}

/// Checks a submitted What's the Nuts answer.
///
/// Returns `410 Gone` when the problem id is unknown -- never a graded answer.
pub async fn check_nuts_answer(
    State(app_state): State<AppState>,
    Json(payload): Json<NutsAnswerRequest>,
) -> Result<Json<NutsAnswerResponse>, ApiError> {
    let problem = app_state
        .nuts_cache
        .get(&payload.problem_id)
        .ok_or(ApiError::ProblemGone(payload.problem_id))?;

    app_state.nuts_cache.invalidate(&payload.problem_id);

    Ok(Json(NutsAnswerResponse {
        correct: payload.selected_index == problem.correct_index,
        correct_index: problem.correct_index,
        correct_hand: problem.candidates[problem.correct_index].clone(),
        within_time_limit: payload.response_time_ms <= problem.time_limit_ms,
    }))
}
