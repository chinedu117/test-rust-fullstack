use jsonwebtoken::{encode, EncodingKey, Header};
use log::debug;
use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub(crate) id: String,
    pub(crate) sub: String,
    pub(crate) app: String,
    pub(crate) exp: usize,
    pub(crate) secret: String
}


impl Claims {
    pub fn encode(&self) -> jsonwebtoken::errors::Result<String> {
        debug!("Encode using {}", self.secret);
        encode(&Header::default(), self, &EncodingKey::from_secret(self.secret.as_ref()))
    }
}
