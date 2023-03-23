#[derive(Debug, Clone)]
pub struct Config {
    pub client_origin: String,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_max_age: i64,
    pub google_oauth_client_id: String,
    pub google_oauth_client_secret: String,
    pub google_oauth_redirect_url: String,
    pub google_oauth_callback_url: String,
    pub google_oauth_url: String,
    pub google_token_url: String,
    pub google_revoke_url: String,
    pub google_profile_url: String,
    pub db_url: String,    
}

impl Config {
    pub fn init() -> Config {
        let client_origin = std::env::var("CLIENT_ORIGIN").expect("CLIENT_ORIGIN must be set");
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_expires_in =
            std::env::var("TOKEN_EXPIRED_IN").expect("TOKEN_EXPIRED_IN must be set");
        let jwt_max_age = std::env::var("TOKEN_MAXAGE").expect("TOKEN_MAXAGE must be set");
        let google_oauth_client_id =
            std::env::var("GOOGLE_OAUTH_CLIENT_ID").expect("GOOGLE_OAUTH_CLIENT_ID must be set");
        let google_oauth_client_secret = std::env::var("GOOGLE_OAUTH_CLIENT_SECRET")
            .expect("GOOGLE_OAUTH_CLIENT_SECRET must be set");
        let google_oauth_redirect_url = std::env::var("GOOGLE_OAUTH_REDIRECT_URL")
            .expect("GOOGLE_OAUTH_REDIRECT_URL must be set");
        let google_oauth_callback_url = std::env::var("GOOGLE_OAUTH_CALLBACK_URL")
            .expect("GOOGLE_OAUTH_CALLBACK_URL must be set");
        let google_oauth_url = std::env::var("GOOGLE_OAUTH_URL")
            .expect("GOOGLE_OAUTH_URL must be set");
        let google_token_url = std::env::var("GOOGLE_TOKEN_URL")
            .expect("GOOGLE_TOKEN_URL must be set");
        let google_revoke_url = std::env::var("GOOGLE_REVOKE_URL")
            .expect("GOOGLE_REVOKE_URL must be set");
        let google_profile_url = std::env::var("GOOGLE_PROFILE_URL")
            .expect("GOOGLE_PROFILE_URL must be set");
        let db_url = std::env::var("DB_URL")
            .expect("DB_URL must be set");
        Config {
            client_origin,
            jwt_secret,
            jwt_expires_in,
            jwt_max_age: jwt_max_age.parse::<i64>().unwrap(),
            google_oauth_client_id,
            google_oauth_client_secret,
            google_oauth_redirect_url,
            google_oauth_callback_url,
            google_oauth_url,
            google_token_url,
            google_revoke_url,
            google_profile_url,
            db_url,
        }
    }
}