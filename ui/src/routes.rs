use shared_models::organization::OrganizationUi;
use shared_models::user::UserUi;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::window;
use yew::{html, Html};
use yew_router::prelude::*;
use crate::components::crud::list::CrudList;
use crate::components::home::Home;
use crate::components::auth::Auth;
use crate::components::logout::Logout;
use crate::components::forms::organization::OrganizationForm;



#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,    
    #[at("/users")]
    Users,
    #[at("/organizations")]
    Organizations,
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
    let organization_form = html! { <OrganizationForm /> };
    match routes {
        Route::Home => html! { <><Home /></> },        
        Route::Users => html! { <CrudList<UserUi> url={"/users/"} title={"Users"} /> },
        Route::Organizations => html! { <CrudList<OrganizationUi> url={"/organizations/"} title={"Organizations"} form={organization_form} /> },
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


