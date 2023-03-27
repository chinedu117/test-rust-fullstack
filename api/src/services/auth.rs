use axum::{extract::FromRequestParts, http::{header, request::Parts}};
use async_trait::async_trait;
use axum::response::{IntoResponse, Response};
use jsonwebtoken::{decode, DecodingKey, Validation};
use tracing::{event, Level};
use regex::Regex;
use crate::config::Config;
use crate::services::error_handler::{ApiError, ApiErrorType};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, EncodingKey, Header};
use jsonwebtoken::errors::Error;


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub(crate) id: String,
    pub(crate) sub: String,
    pub(crate) app: String,
    pub(crate) exp: usize,
    pub(crate) secret: String
}


impl Claims {

    pub fn decode(token: String) -> Result<Self, Error> {
        let config = Config::init();
        let decode = decode::<Claims>(&token,
                                      &DecodingKey::from_secret(config.jwt_secret.as_ref()), &Validation::default());
        match decode {
            Ok(token_data) => {
                let claims = token_data.claims;
                Ok(claims)
            }
            Err(err) => {
                event!(Level::INFO, "Error Decoding: {}", err);
                Err(err)
            }
        }
    }

    pub fn encode(&self) -> jsonwebtoken::errors::Result<String> {
        encode(&Header::default(), self, &EncodingKey::from_secret(self.secret.as_ref()))
    }
}


// An extractor that performs authorization.
pub struct BearerAuth;

impl BearerAuth {
    fn default_error(msg: String) -> Response {
        let error = ApiError {kind: ApiErrorType::Unauthorized, msg};
        error.into_response()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for BearerAuth
    where
        S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok());
        event!(Level::INFO, "auth header: {:?}", auth_header);
        match auth_header {
            Some(value) => {
                let re = Regex::new(r"^(?i)Bearer (.*)(?-i)").unwrap();
                match re.is_match(value) {
                    true => {
                        event!(Level::INFO, "valid bearer token {}", value);
                        let value = value.trim().replace("Bearer ", "");
                        event!(Level::INFO, "token value {}", value);
                        match Claims::decode(value.to_string()) {
                            Ok(claims) => {
                                event!(Level::INFO, "claims: {:?}", claims);
                                Ok(Self)
                            }
                            Err(err) => {
                                let err = format!("Error Decoding: {}", err);
                                event!(Level::INFO, err);
                                Err(Self::default_error(err))
                            }
                        }
                    }
                    false => {
                        let err = format!("invalid bearer token: {}", value);
                        event!(Level::INFO, err);
                        Err(Self::default_error(err))
                    }
                }
            }
            _ => Err(Self::default_error("no token found in request".to_string())),
        }
    }
}