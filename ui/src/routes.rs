use wasm_bindgen::UnwrapThrowExt;
use web_sys::window;
use yew::{html, Html};
use yew_router::prelude::*;
use crate::components::crud::list::CrudList;
use crate::components::home::Home;
use crate::components::auth::Auth;
use crate::components::logout::Logout;
use shared_models::user::UserModelUi;


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,    
    #[at("/users")]
    Users,
    #[at("/auth/logout")]
    Logout,
    #[at("/auth/login")]
    Login,
    #[at("/auth/:token")]
    AuthToken {token: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <><Home /></> },        
        Route::Users => html! { <CrudList<UserModelUi> url={"/users/"} title={"Users"} /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
        Route::Logout => html! { <Logout /> },
        Route::Login => {
            let location = window()
                .expect_throw("window is undefined")
                .location();
            match location.set_href("http://localhost:3000/auth/login") {
                Ok(_) => html! {<></>},
                Err(_) => html! {<>{"Cannot redirect to login"}</>},
            }
            
        },
        Route::AuthToken { token } => html! {<Auth token={token} /> }
    }
}


