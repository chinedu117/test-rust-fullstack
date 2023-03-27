use axum::Router;
use crate::AppState;

pub trait ResourceRoutes {
    fn export_routes(state: AppState) -> Router;
}