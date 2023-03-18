use gloo_net::http::{Request, Method};
use gloo_net::Error;
use wasm_bindgen::{JsValue, UnwrapThrowExt};
use gloo_net::http::Response;
use yew::Callback;
use yewdux::prelude::Dispatch;
use crate::stores::user::UserState;
use web_sys::window;


pub struct Client {
    pub url: String,
    pub method: Method,
    pub body: Option<JsValue>,
    pub state: UserState,
    pub dispatch: Dispatch<UserState>
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
        let token = &self.state.token;
        let req = Request::new(format!("http://localhost:3000{}", self.url).as_str())
            .method(self.method)
            .header("Authorization", &format!("Bearer {}", token).to_string());
        req
    }

    fn handle_unauth(&self) -> Result<Response, Error> {
        let cb: Callback<UserState> = self.dispatch.reduce_mut_callback(move |state| {
            state.logged = false;
            state.token = Default::default();
        });
        cb.emit(Default::default());
        let location = window().expect_throw("window is undefined").location();
        location.set_href("http://localhost:8080/auth/logout").unwrap();
        Err(Error::GlooError("Not Authorized".to_string()))
    }

    pub async fn send_request(&self) -> Result<Response, Error> {        
        let req = self.create_request();
        let req = self.add_body(req);
        match req.send().await {
            Ok(res) => {
                match res.status() {
                    401 => { self.handle_unauth() }
                    _ => Ok(res)
                }
            },
            Err(err) => Err(err)
        }
    }
}

