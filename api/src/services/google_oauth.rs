use reqwest::{Client as HttpClient};
use oauth2::{
    basic::{BasicClient, BasicErrorResponseType, BasicTokenType},
    Client, EmptyExtraTokenFields, RedirectUrl, RevocationErrorResponseType,
    RevocationUrl, Scope, StandardErrorResponse, StandardRevocableToken, StandardTokenIntrospectionResponse,
    StandardTokenResponse, TokenUrl, AuthorizationCode, AuthUrl, ClientId, ClientSecret, CsrfToken,
    reqwest::{async_http_client}, url::Url
};
use oauth2::{RequestTokenError, TokenResponse};
use serde::{Deserialize, Serialize};
use crate::Config;


#[derive(Deserialize, Serialize)]
pub struct GoogleUserResult {
    pub id: String,
    pub picture: String,
    pub email: String
}

#[derive(Debug, Clone)]
pub struct GoogleAuth {
    pub google_profile_url: String,
    pub client: Client<StandardErrorResponse<BasicErrorResponseType>,
        StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
        BasicTokenType,
        StandardTokenIntrospectionResponse<EmptyExtraTokenFields, BasicTokenType>,
        StandardRevocableToken,
        StandardErrorResponse<RevocationErrorResponseType>>
}

impl GoogleAuth {
    pub fn new() -> Self {
        let config = Config::init();
        let google_redirect_url = Url::parse(&config.google_oauth_callback_url).expect("Invalid URL");
        Self {google_profile_url: config.google_profile_url, client: BasicClient::new(
            ClientId::new(config.google_oauth_client_id),
            Some(ClientSecret::new(config.google_oauth_client_secret)),
            AuthUrl::new(config.google_oauth_url).expect("Invalid AUTH URL"),
            Some(TokenUrl::new(config.google_token_url).expect("Invalid TOKEN URL")),
        )
        .set_redirect_uri(RedirectUrl::new(String::from(google_redirect_url))
            .expect("Invalid URL"))
        .set_revocation_uri(RevocationUrl::new(config.google_revoke_url)
            .expect("Invalid URL")) }
    }

    pub fn get_auth_url(&self) -> String {
        let (authorize_url, _csrf_state) = self.client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/calendar".parse().unwrap(),
            ))
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/plus.me".to_string(),
            ))
            .add_scope(Scope::new(
                "https://www.googleapis.com/auth/userinfo.email".to_string(),
            ))
            .url();
        authorize_url.to_string()
    }

    pub async fn get_token(&self, code: AuthorizationCode) -> Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, String> {
        let token_result = self.client
            .exchange_code(code)
            .request_async(async_http_client).await;
    
        match token_result {
            Ok(token) => {
                Ok(token)
            }
            Err(err) => {
                match err {
                    RequestTokenError::ServerResponse(s) => Err(s.to_string()),
                    RequestTokenError::Request(r) => Err(r.to_string()),
                    RequestTokenError::Parse(e, _) => Err(e.to_string()),
                    RequestTokenError::Other(o) => Err(o.to_string())
                }
            }
        }
    }
    
    pub async fn get_user_info(&self, response: StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>) -> Result<GoogleUserResult, String> {
        let client = HttpClient::new();
        let url = Url::parse(&self.google_profile_url).unwrap();
        let response = client.get(url).bearer_auth(response.access_token().secret()).send().await;
        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    match resp.json::<GoogleUserResult>().await {
                        Ok(user) => Ok(user),
                        Err(err) => Err(err.to_string()),
                    }
                } else {
                    Err("cannot access profile".to_string())
                }
            },
            Err(err) => Err(err.to_string()),
        }
    }
}

