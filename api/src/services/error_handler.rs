use std::fmt::{Debug, Display, Formatter, Result};
use actix_web::{error, http::{StatusCode}, HttpResponse, };
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JsonFormatedError {
    error: String
}

#[derive(Debug)]
pub struct ApiError {
    pub(crate) kind: ApiErrorType,
    pub(crate) msg: String
}


#[derive(Debug)]
pub enum ApiErrorType {
    InternalError,
    BadClientData,
    Timeout,
    NotFound
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

impl error::ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let formatted_error = JsonFormatedError { error: self.msg.clone() };
        HttpResponse::build(self.status_code())
            .json(formatted_error)
    }

    fn status_code(&self) -> StatusCode {
        let cloned = self.clone();
        match cloned {
            ApiError { kind, msg: _ } => {
                match kind {
                    ApiErrorType::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
                    ApiErrorType::BadClientData => StatusCode::BAD_REQUEST,
                    ApiErrorType::NotFound => StatusCode::NOT_FOUND,
                    ApiErrorType::Timeout => StatusCode::GATEWAY_TIMEOUT
                }
            }
        }
    }
}