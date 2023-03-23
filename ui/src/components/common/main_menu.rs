use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::use_store;
use crate::{Route, stores::user::UserState};
use ybc::NavbarFixed::Top;


#[function_component(MainMenu)]
pub fn main_menu() -> Html {
    
    let (state, _) = use_store::<UserState>();
    let brand = html! {
        <div class={"navbar-brand"}>
            <span class="icon-text">
                <span class="icon">
                    <i class="fas fa-rocket"></i>
                </span>
                <span>{"Test Rust - UI"}</span>
                </span>
        </div>
    };
    let navstart = html! {
            <>        
                <Link<Route> to={Route::Home} classes={"navbar-item"}>{"Home"}</Link<Route>>
                if state.logged {
                    <Link<Route> to={Route::Users} classes={"navbar-item"}>{"Users"}</Link<Route>>
                    <Link<Route> to={Route::Organizations} classes={"navbar-item"}>{"Organizations"}</Link<Route>>
                    <Link<Route> to={Route::Logout} classes={"navbar-item"}>{"Logout"}</Link<Route>>
                } else {
                    <Link<Route> to={Route::Login} classes={"navbar-item"}>{"Login"}</Link<Route>>
                }
            </>
    };
    html! {
        <div>
            <ybc::Navbar fixed={Top} navbrand={brand} navstart={navstart} />
        </div>

    }
}