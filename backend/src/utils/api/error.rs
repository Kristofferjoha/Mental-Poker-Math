use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json
};
use serde::Serialize;
use tracing::warn;
use uuid::Uuid;

#[derive(Serialize)]
pub struct ApiErrorBody {
    pub error: &'static str,
    pub message: String
}

#[derive(Debug)]
pub enum ApiError {
    ProblemGone(Uuid),
    InvalidParameter { name: &'static str, detail: String }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
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
                        )
                    })
                )
                    .into_response()
            }
            ApiError::InvalidParameter { name, detail } => (
                StatusCode::BAD_REQUEST,
                Json(ApiErrorBody {
                    error: "invalid_parameter",
                    message: format!("Query parameter `{name}` is invalid: {detail}")
                })
            )
                .into_response()
        }
    }
}
