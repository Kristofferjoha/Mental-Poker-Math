//! Shared error type for the API handlers.
//!
//! Exists so that a cache miss can never be mistaken for a graded answer. The
//! handlers previously returned `200 OK` with zeroed fields when a problem id was
//! unknown, which told the player they were wrong and that the true equity was
//! 0% -- a confident, plausible, and completely fabricated lesson.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use tracing::warn;
use uuid::Uuid;

/// Body returned for a failed API call.
#[derive(Serialize)]
pub struct ApiErrorBody {
    /// Stable machine-readable code; the frontend branches on this, not the prose.
    pub error: &'static str,
    /// Human-readable explanation, safe to show to a player.
    pub message: String,
}

#[derive(Debug)]
pub enum ApiError {
    /// The problem id is not in the cache. It expired, was already answered, or
    /// was lost when the server restarted.
    ProblemGone(Uuid),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            // 410 rather than 404: the id was almost certainly issued by this server
            // at some point, and the client should stop retrying it and ask for a
            // fresh problem instead.
            ApiError::ProblemGone(id) => {
                warn!("Problem id not found or expired: {}", id);
                (
                    StatusCode::GONE,
                    Json(ApiErrorBody {
                        error: "problem_gone",
                        message: format!(
                            "Problem {id} is no longer available. It may have expired, \
                             already been answered, or been lost to a server restart. \
                             Request a new problem."
                        ),
                    }),
                )
                    .into_response()
            }
        }
    }
}
