use shared_models::user::{Entity, ActiveModel, ModelWithoutId, Column};
use axum::{Json, middleware, Router};
use axum::routing::{get};
use macros::CrudRoutes;

use crate::AppState;
use crate::routes::resource_routes::ResourceRoutes;
use crate::services::auth::BearerAuth;

#[derive(CrudRoutes)]
pub struct UserRoutes {}

impl ResourceRoutes for UserRoutes {
    fn export_routes(state: AppState) -> Router {
        Router::new()
            .route("/users/", get(Self::list).post(Self::create))
            .route("/users/:id", get(Self::get)
                .delete(Self::delete).patch(Self::update))            
            .with_state(state.clone())
            .route_layer(middleware::from_extractor::<BearerAuth>())

    }
}