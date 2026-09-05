use axum::{
    extract::{Query, State},
    Json
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::poker_core::card::Card;
use crate::problems::king_of_the_hill::{generate, score, MAX_HANDS, MIN_HANDS};
use crate::utils::api::{params, ApiError};
use crate::utils::app_state::AppState;

#[derive(Serialize)]
pub struct KingOfHillProblemResponse {
    pub problem_id: Uuid,
    pub board: Vec<Card>,
    pub hands: Vec<Vec<Card>>,
    #[serde(rename = "numHands")]
    pub num_hands: usize
}

#[derive(Deserialize)]
pub struct KingOfHillAnswerRequest {
    #[serde(rename = "problemId")]
    pub problem_id: Uuid,
    #[serde(rename = "orderedIndices")]
    pub ordered_indices: Vec<usize>
}

#[derive(Serialize)]
pub struct KingOfHillAnswerResponse {
    #[serde(rename = "correctOrder")]
    pub correct_order: Vec<usize>,
    pub equities: Vec<f64>,
    #[serde(rename = "correctPairs")]
    pub correct_pairs: usize,
    #[serde(rename = "totalPairs")]
    pub total_pairs: usize,
    #[serde(rename = "kendallTau")]
    pub kendall_tau: f64,
    pub perfect: bool
}

pub async fn generate_king_of_the_hill_problem(
    State(app_state): State<AppState>,
    Query(params): Query<HashMap<String, String>>
) -> Result<Json<KingOfHillProblemResponse>, ApiError> {
    let num_hands = match params.get("numHands") {
        None => MIN_HANDS,
        Some(raw) => {
            let parsed: usize = raw.parse().map_err(|_| ApiError::InvalidParameter {
                name: "numHands",
                detail: format!("expected a whole number, got {raw:?}")
            })?;
            if !(MIN_HANDS..=MAX_HANDS).contains(&parsed) {
                return Err(ApiError::InvalidParameter {
                    name: "numHands",
                    detail: format!("must be between {MIN_HANDS} and {MAX_HANDS}, got {parsed}")
                });
            }
            parsed
        }
    };

    let allowed_streets = params::streets(&params, &["pre-flop", "flop", "turn"])?;

    let problem = generate(num_hands, allowed_streets, &app_state.seven_card_tables);
    let problem_id = Uuid::new_v4();
    app_state.koth_cache.insert(problem_id, problem.clone());

    Ok(Json(KingOfHillProblemResponse {
        problem_id,
        board: problem.board,
        hands: problem.hands,
        num_hands
    }))
}

pub async fn check_king_of_the_hill_answer(
    State(app_state): State<AppState>,
    Json(payload): Json<KingOfHillAnswerRequest>
) -> Result<Json<KingOfHillAnswerResponse>, ApiError> {
    let problem = app_state
        .koth_cache
        .get(&payload.problem_id)
        .ok_or(ApiError::ProblemGone(payload.problem_id))?;

    let n = problem.hands.len();
    let mut seen = vec![false; n];
    let valid = payload.ordered_indices.len() == n
        && payload.ordered_indices.iter().all(|&i| {
            i < n && !std::mem::replace(&mut seen[i], true)
        });
    if !valid {
        return Err(ApiError::InvalidParameter {
            name: "orderedIndices",
            detail: format!("must be each of 0..{n} exactly once, got {:?}", payload.ordered_indices)
        });
    }

    app_state.koth_cache.invalidate(&payload.problem_id);

    let result = score(&payload.ordered_indices, &problem.equities);
    Ok(Json(KingOfHillAnswerResponse {
        correct_order: problem.correct_order.clone(),
        equities: problem.equities.clone(),
        correct_pairs: result.correct_pairs,
        total_pairs: result.total_pairs,
        kendall_tau: result.kendall_tau,
        perfect: result.correct_pairs == result.total_pairs
    }))
}
