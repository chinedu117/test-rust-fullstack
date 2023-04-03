use gloo_net::http::{Request, Method};
use serde::{Serialize, Deserialize};
use wasm_bindgen::{JsValue, UnwrapThrowExt};
use gloo_net::http::Response;
use web_sys::window;
use crate::components::messages::{Message, MessageType};
use crate::services::localstorage::UserState;

use super::localstorage::UserContext;

pub struct Client {
    pub url: String,
    pub method: Method,
    pub body: Option<JsValue>,
    pub state: UserState    
}

impl Client {

    fn add_body(&self, req: Request) -> Request {
        match self.method {
            Method::GET => req,
            Method::HEAD => req,
            Method::POST => req.body(self.body.clone().unwrap()),
            Method::PUT => req,
            Method::DELETE => req,
            Method::CONNECT => req,
            Method::OPTIONS => req,
            Method::TRACE => req,
            Method::PATCH => req.body(self.body.clone().unwrap()),
        }
    }

    fn create_request(&self) -> Request {
        let token = &self.state.token.clone().unwrap_or_default();
        Request::new(format!("http://localhost:3000{}", self.url).as_str())
            .method(self.method)
            .header("Authorization", &format!("Bearer {}", token).to_string())
        
    }

    fn handle_unauth(&self) -> Result<Response, Message> {        
        let location = window().expect_throw("window is undefined").location();
        location.set_pathname("/logout").unwrap();
        Err(Message::new("Unauthorized".to_string(), MessageType::AuthError))
    }

    async fn send_request(&self) -> Result<Response, Message> {        
        let req = self.create_request();
        let req = self.add_body(req);
        let req = req.header("Content-Type", "application/json");
        match req.send().await {
            Ok(res) => {
                match res.status() {
                    401 => { self.handle_unauth() }
                    200 | 202 => Ok(res),                    
                    404 => {                      
                        Err(Message::new("Resource Not Found".to_string(), MessageType::NotFound))
                    },
                    _ => {
                        let msg = res.text().await.unwrap_or_default();
                        Err(Message::new(msg, MessageType::ApiError))
                    }
                }
            },
            Err(err) => {
                Err(Message::new(err.to_string(), MessageType::ApiError))
            }
        }
    }

    pub async fn fetch_all<M>(_filter: String, url: &String) -> Result<Vec<M>, Message> where M: Serialize + for<'a> Deserialize<'a> {
        let client = Client {
            url: url.to_string(),
            method: Method::GET,
            body: None,
            state: UserContext::new().state
        };
        match client.send_request().await {
            Ok(res) => {
                let users: Vec<M> = res.json().await.unwrap();
                Ok(users)
            },
            Err(err) => {
               Err(err)
            }
        }
    }

    pub async fn fetch_one<M>(id: i32, url: &String) -> Result<M, Message> where M: Serialize + for<'a> Deserialize<'a> {
        let client = Client {
            url: format!("{}{}", url.to_string(), id),
            method: Method::GET,
            body: None,
            state: UserContext::new().state
        };
        match client.send_request().await {
            Ok(res) => {
                let user: M = res.json().await.unwrap();
                Ok(user)
            },
            Err(err) => {
                Err(err)
            }
        }
    }

    pub async fn delete_one(url: &String, id: i32) -> Result<u16, Message>  {
        let client = Client {
            url: format!("{}{}",url.to_string(), id),
            method: Method::DELETE,
            body: None,
            state: UserContext::new().state
        };
        match client.send_request().await {
            Ok(res) => {
                Ok(res.status())
            },
            Err(err) => {
               Err(err)
            }
        }
    }

    pub async fn update_one(url: &String, id: i32, body: JsValue) -> Result<u16, Message> {
        let client = Client {
            url: format!("{}{}",url.to_string(), id),
            method: Method::PATCH,
            body: Some(body),
            state: UserContext::new().state
        };
        match client.send_request().await {
            Ok(res) => {
                Ok(res.status())
            },
            Err(err) => {
                Err(err)
            }
        }
    }

    pub async fn create_one(url: &String, body: JsValue) -> Result<u16, Message> {
        let client = Client {
            url: url.to_string(),
            method: Method::POST,
            body: Some(body),
            state: UserContext::new().state
        };
        match client.send_request().await {
            Ok(res) => {
                Ok(res.status())
            },
            Err(err) => {
               Err(err)
            }
        }
    }
}

