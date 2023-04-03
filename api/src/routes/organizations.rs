use shared_models::organization::{Entity, ActiveModel, ModelWithoutId, Column};
use axum::{Json, Router};
use axum::middleware::FromExtractorLayer;
use axum::routing::{get};
use macros::CrudRoutes;

use crate::AppState;
use crate::routes::resource_routes::ResourceRoutes;
use crate::services::auth::BearerAuth;

#[derive(CrudRoutes)]
pub struct OrganizationRoutes {}

impl ResourceRoutes for OrganizationRoutes {
    fn export_routes(state: AppState, auth: FromExtractorLayer<BearerAuth, ()>) -> Router {
        Router::new()
            .route("/organizations/", get(Self::list).post(Self::create))
            .route("/organizations/:id", get(Self::get)
                .delete(Self::delete).patch(Self::update))            
            .with_state(state.clone())
            .route_layer(auth)

    }
}