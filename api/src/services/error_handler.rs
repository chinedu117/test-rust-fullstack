use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub struct ApiError {
    pub(crate) kind: ApiErrorType,
    pub(crate) msg: String
}

impl ApiError {
    pub fn create_response(kind: ApiErrorType, msg: String) -> Response {
        Self { kind, msg }.into_response()
    }
}


#[derive(Debug)]
pub enum ApiErrorType {
    InternalError,
    BadClientData,
    Unauthorized,
    NotFound
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ApiError { msg, kind } => {
                match kind {
                    ApiErrorType::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, msg),
                    ApiErrorType::BadClientData => (StatusCode::BAD_REQUEST, msg),
                    ApiErrorType::NotFound => (StatusCode::NOT_FOUND, msg),
                    ApiErrorType::Unauthorized => (StatusCode::UNAUTHORIZED, msg)
                }
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}