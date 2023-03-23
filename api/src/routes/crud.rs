use actix_web::Scope;


pub trait DefaultRoutes {
    fn export_routes() -> Scope;
}


