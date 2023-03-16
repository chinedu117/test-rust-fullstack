use gloo_net::http::{Request, Method};
use gloo_net::Error;
use wasm_bindgen::{JsValue, UnwrapThrowExt};
use gloo_net::http::Response;
use yew::Callback;
use yewdux::prelude::Dispatch;
use crate::stores::user::UserState;
use web_sys::window;

pub struct Api {}

impl Api {
    pub async fn send_request(url: &String, method: Method, body: Option<JsValue>, 
        state: UserState, dispatch: Dispatch<UserState>) -> Result<Response, Error> {        
        let token = &state.token;
        let req = Request::new(format!("http://localhost:3000{}", url).as_str())
        .method(method)                
        .header("Authorization", &format!("Bearer {}", token).to_string());
        let req = match method {
            Method::GET => req,
            Method::HEAD => req,
            Method::POST => req.body(body.unwrap()),
            Method::PUT => req,
            Method::DELETE => req,
            Method::CONNECT => req,
            Method::OPTIONS => req,
            Method::TRACE => req,
            Method::PATCH => req.body(body.unwrap()),
        };
        match req.send().await {
            Ok(res) => {
                match res.status() {
                    401 => {
                        let cb: Callback<UserState> = dispatch.reduce_mut_callback(move |state| {
                            state.logged = false;
                            state.token = Default::default();
                        });
                        cb.emit(Default::default());
                        let location = window().expect_throw("window is undefined").location();
                        location.set_href("http://localhost:8080/auth/logout").unwrap();
                        Err(Error::GlooError("Not Authorized".to_string()))
                    }
                    _ => Ok(res)
                }
            },
            Err(err) => Err(err)
        }
    }
}

