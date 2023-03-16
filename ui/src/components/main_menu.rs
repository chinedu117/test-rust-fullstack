use yew::prelude::*;
use yew_router::prelude::*;
use yew_bootstrap::component::{NavBar, NavItem, BrandType};
use yewdux::prelude::use_store;
use crate::{Route, stores::user::UserState};


#[function_component(MainMenu)]
pub fn nav_bar() -> Html {
    let brand = BrandType::BrandIcon {
        text: AttrValue::from("Test Rust Actix and Yew"),
        url: Some(AttrValue::from("http://localhost:8080")),
        icon: AttrValue::from("rocket")
    };
    let (state, _) = use_store::<UserState>();
    html! {
        <div>
            <NavBar nav_id={"test-nav"} class="navbar navbar-expand-lg navbar-light bg-light" brand={brand}>
                
                <Link<Route> to={Route::Home}><NavItem text="Home" /></Link<Route>>
                
                if state.logged {
                    <Link<Route> to={Route::Users}><NavItem text="Users" /></Link<Route>>
                    <Link<Route> to={Route::Logout}><NavItem text="Logout" /></Link<Route>>
                } else {
                    <Link<Route> to={Route::Login}><NavItem text="Login" /></Link<Route>>
                }
                
            </NavBar>
        </div>

    }
}