use std::time::Duration;
use sea_orm::{Database, ConnectOptions, DatabaseConnection, DbErr};


pub struct DatabaseClient {
    pub url: String
}

impl DatabaseClient {

     pub async fn init(url: String) -> Result<DatabaseConnection, DbErr> {
        let new = Self {url};
        new.create_pool().await
    }

    
    async fn create_pool(&self) -> Result<DatabaseConnection, DbErr> {
        let mut opt = ConnectOptions::new(self.url.to_owned());
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true);
    
        Database::connect(opt).await
    
    }

}
