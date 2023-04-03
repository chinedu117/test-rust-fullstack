use axum::extract::{Query, State};
use axum::middleware::FromExtractorLayer;
use axum::response::{IntoResponse, Redirect, Response};
use axum::Router;
use axum::routing::get;
use crate::AppState;
use crate::routes::resource_routes::ResourceRoutes;
use serde::{Deserialize, Serialize};
use crate::config::Config;
use oauth2::{AuthorizationCode};
use crate::services::auth::{BearerAuth, Claims};
use shared_models::user;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, IntoActiveModel};
use sea_orm::ActiveValue::Set;
use crate::services::error_handler::{ApiError, ApiErrorType};


#[derive(Deserialize, Serialize)]
pub struct OAuthCallbackParams {
    pub code: String,
    pub state: String,
    pub scope: String
}

pub struct AuthRoutes;

impl AuthRoutes {
    
    fn default_error(msg: String) -> Response {
        ApiError::create_response(ApiErrorType::InternalError, msg)       
    }

    async fn find_or_register(user: Claims, db: &DatabaseConnection) -> Result<user::ActiveModel, DbErr> {
        match user::Entity::find().filter(user::Column::Username.contains(user.sub.as_str())).one(&db.clone()).await {
            Ok(result) => {
                match result {
                    None => {
                        let model = user::ActiveModel {
                            id: Default::default(),
                            username: Set(user.sub),
                            service: Set("google".to_string()),
                        };
                        match model.save(db).await {
                            Ok(saved) => {
                                Ok(saved)
                            }
                            Err(err) => Err(err)
                        }
                    }
                    Some(model) => Ok(model.into_active_model())
                }
            }
            Err(err) => Err(err)
        }
    }

    async fn login(State(state): State<AppState>) -> Redirect {
        let auth_url = state.oauth.get_auth_url();
        Redirect::to(&auth_url)
    }


    async fn callback(State(state): State<AppState>, Query(query):  Query<OAuthCallbackParams>,) -> impl IntoResponse {
        let code = AuthorizationCode::new(query.code.clone());
        let token = state.oauth.get_token(code).await;
        let config = Config::init();
        match token {
            Ok(t) => {
                let info = state.oauth.get_user_info(t).await;
                match info {
                    Ok(profile) => {
                        let claim = Claims {
                            id: profile.id.to_string(),
                            sub: profile.email.to_string(),
                            app: "api".to_string(),
                            exp: 10000000000,
                            secret: config.jwt_secret
                        };
                        let token = claim.encode();
                        match Self::find_or_register(claim, &state.db).await {
                            Ok(_) => {
                                Redirect::to(&format!("http://localhost:8080/auth/{}", token.unwrap())).into_response()
                            }
                            Err(err) => {
                                Self::default_error(err.to_string())
                            }
                        }
                    }
                    Err(err) => {
                        Self::default_error(err.to_string())
                    }
                }
            }
            Err(err) => {
                Self::default_error(err.to_string())
            }
        }
    }
}

impl ResourceRoutes for AuthRoutes {
    fn export_routes(state: AppState, _auth: FromExtractorLayer<BearerAuth, ()>) -> Router {
        Router::new()
            .route("/auth/login", get(Self::login)).with_state(state.clone())
            .route("/auth/google/callback", get(Self::callback)).with_state(state.clone())
    }
}

