use macros::GenerateCrudRoutes;
use shared_models::organization::{Entity, ActiveModel, ModelWithoutId, Column};
use actix_web::{HttpResponse, ResponseError, Scope, web};
use actix_web::http::StatusCode;
use sea_orm::{ActiveModelTrait, DbErr, EntityOrSelect, EntityTrait, IntoActiveModel, Value};
use sea_orm::sea_query::ValueTuple;
use crate::AppState;
use crate::services::error_handler::{ApiError, ApiErrorType};
use super::crud::DefaultRoutes;

#[derive(GenerateCrudRoutes)]
pub struct OrganizationRoutes {}

impl DefaultRoutes for OrganizationRoutes {
    fn export_routes() -> Scope {
        web::scope("/organizations")
            .route("/", web::get().to(Self::list))
            .route("/", web::post().to(Self::create))
            .route("/{id}/", web::delete().to(Self::delete))
            .route("/{id}/", web::get().to(Self::get))
            .route("/{id}/", web::patch().to(Self::update))

    }    
}
