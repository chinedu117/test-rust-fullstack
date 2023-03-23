extern crate log;
extern crate dotenv;

mod services;
mod routes;
mod env_config;

use dotenv::dotenv;
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use actix_cors::Cors;
use env_logger;
use sea_orm::{DatabaseConnection, DbErr};
use actix_web_httpauth::middleware::HttpAuthentication;
use env_config::Config;

use services::database_client::DatabaseClient;
use services::google_auth::GoogleAuth;
use routes::users::UserRoutes;
use routes::organizations::OrganizationRoutes;
use routes::crud::DefaultRoutes;
use routes::auth::AuthRoutes;

pub struct AppState {
    db: DatabaseConnection,
    oauth: GoogleAuth
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        let cloned = self.to_owned();
        Self {db: cloned.db, oauth: cloned.oauth}
    }
}

async fn create_state(config: Config) -> Result<AppState, DbErr> {
    let db_client = DatabaseClient::new(config.db_url);
    match db_client.create_pool().await {
        Ok(db) => {
            let oauth = GoogleAuth::new();            
            Ok(AppState {db, oauth})
        }
        Err(err) => {
            Err(err)
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    dotenv().ok();
    println!("Server starting in port 3000");
    HttpServer::new( move || {
        let cors = Cors::permissive()
            .max_age(3600);
        let auth = HttpAuthentication::bearer(AuthRoutes::bearer_auth_validator);
        let config = Config::init();
        App::new()
            .wrap(Logger::default())                        
            .wrap(cors)
            .data_factory(move || { create_state(config.clone()) })                     
            .service(AuthRoutes::export_routes())
            .service(UserRoutes::export_routes().wrap(auth.clone()))
            .service(OrganizationRoutes::export_routes().wrap(auth.clone()))
    })
        .bind(("0.0.0.0", 3000))?
        .run()
        .await
}