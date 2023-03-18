use std::fmt;
use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use jsonwebtoken::{decode, Validation, DecodingKey};
use log::debug;
use oauth2::{AuthorizationCode};
use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::env_config::Config as EnvConfig;
use actix_web::{HttpResponse, ResponseError, Scope, web, Error};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, IntoActiveModel};
use sea_orm::ActiveValue::Set;
use shared_models::user;


use crate::routes::crud::DefaultRoutes;
use crate::services::jwt::{Claims};
use crate::services::error_handler::{ApiError, ApiErrorType};


#[derive(Deserialize, Serialize)]
pub struct OAuthCallbackParams {
    code: String,
    state: String,
    scope: String
}

impl fmt::Display for OAuthCallbackParams {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Code: {}, state: {}", self.code, self.state)
    }
}

#[derive(Deserialize, Serialize)]
pub struct TokenResponse {
    token: String,
}


pub struct AuthRoutes {}

impl DefaultRoutes for AuthRoutes {
    fn export_routes() -> Scope {
        web::scope("/auth")
            .route("/login", web::get().to(Self::login))
            .route("/google/callback", web::get().to(Self::callback))
    }
}

impl AuthRoutes {
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

    async fn login(data: web::Data<AppState>) -> HttpResponse {
        let auth_url = data.oauth.get_auth_url();
        HttpResponse::Found().append_header(("Location", auth_url)).finish()
    }


    pub async fn callback(query_params: web::Query<OAuthCallbackParams>, data: web::Data<AppState>) -> HttpResponse {
        let code = AuthorizationCode::new(query_params.code.clone());        
        let token = data.oauth.get_token(code).await;
        let config = EnvConfig::init();  
        match token {
            Ok(t) => {
                let info = data.oauth.get_user_info(t).await;
                match info {
                    Ok(profile) => {
                        println!("Profile: {}", profile.id);
                        let claim = Claims {
                            id: profile.id.to_string(),
                            sub: profile.email.to_string(),
                            app: "actix".to_string(),
                            exp: 10000000000,
                            secret: config.jwt_secret
                        };
                        let token = claim.encode();
                        match Self::find_or_register(claim, &data.db).await {
                            Ok(_) => HttpResponse::Found().append_header(("Location", format!("http://localhost:8080/auth/{}", token.unwrap()))).finish(),
                            Err(err) => ApiError { kind: ApiErrorType::InternalError, msg: err.to_string() }.error_response()
                        }

                    },
                    Err(err) => ApiError { kind: ApiErrorType::InternalError, msg: err.to_string() }.error_response()
                }
            }
            Err(err) => ApiError { kind: ApiErrorType::InternalError, msg: err.to_string() }.error_response()
        }
    }
    
    pub async fn bearer_auth_validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
        let resp = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);
        let config = EnvConfig::init();
        debug!("config jwt {}", config.jwt_secret);
        let decode = decode::<Claims>(&credentials.token().to_owned(), 
            &DecodingKey::from_secret(config.jwt_secret.as_ref()), &Validation::default());
        match decode {
            Ok(_) => {
                Ok(req)
            }
            Err(err) => {
                println!("Error Decoding: {}", err);
                Err((AuthenticationError::from(resp).into(), req))
            }
        }
    }
}
