use axum::{Router, middleware::{FromExtractorLayer}};
use crate::AppState;
use crate::services::auth::BearerAuth;

pub trait ResourceRoutes {
    fn export_routes(state: AppState, auth: FromExtractorLayer<BearerAuth, ()>) -> Router;
}