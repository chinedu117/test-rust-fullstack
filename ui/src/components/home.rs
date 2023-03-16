use yew::prelude::*;
use yewdux::prelude::*;
use crate::stores::user::UserState;

#[function_component(Home)]
pub fn home() -> Html {
    let logged = use_selector(|state: &UserState| state.logged.clone());
    html! {
        <div>
            <h1>{ "Home" }</h1>
            <h3>{logged}</h3>
        </div>
    }
}