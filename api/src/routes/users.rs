use macros::GenerateCrudRoutes;
use shared_models::user::{Entity, ActiveModel, ModelWithoutId};
use actix_web::{HttpResponse, ResponseError, Scope, web};
use actix_web::http::StatusCode;
use sea_orm::{ActiveModelTrait, DbErr, EntityOrSelect, EntityTrait, IntoActiveModel, Value};
use sea_orm::sea_query::ValueTuple;
use crate::AppState;
use crate::services::error_handler::{ApiError, ApiErrorType};
use shared_models::user::Column;
use super::crud::DefaultRoutes;

#[derive(GenerateCrudRoutes)]
pub struct UserRoutes {}

