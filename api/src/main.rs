extern crate dotenv;

mod config;
mod services;
mod routes;

use axum::Router;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use crate::config::Config;
use crate::services::database::{DatabaseClient};
use crate::services::google_oauth::GoogleAuth;
use sea_orm::{DatabaseConnection};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use dotenv::dotenv;
use crate::routes::auth::AuthRoutes;
use crate::routes::resource_routes::ResourceRoutes;
use crate::routes::users::UserRoutes;
use crate::routes::organizations::OrganizationRoutes;

#[derive(Debug, Clone)]
pub struct AppState {
    db: DatabaseConnection,
    oauth: GoogleAuth
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::registry().with(tracing_subscriber::fmt::layer()).init();
    let config = Config::init();
    let db_client = DatabaseClient::new(config.db_url);
    let db_pool = db_client.create_pool().await.expect("Failed to create database pool");
    let app_state = AppState {db: db_pool.clone(), oauth: GoogleAuth::new() };
    let auth_routes = AuthRoutes::export_routes(app_state.clone());
    let user_routes = UserRoutes::export_routes(app_state.clone());
    let org_routes = OrganizationRoutes::export_routes(app_state.clone());
    let app = Router::new()
        .merge(auth_routes)
        .merge(user_routes)
        .merge(org_routes)
        .layer(CorsLayer::permissive());

    serve(app, config.app_port).await;

}

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


