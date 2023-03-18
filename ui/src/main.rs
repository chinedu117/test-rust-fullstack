mod components;
mod stores;
mod api;
mod routes;
use yew_router::prelude::*;
use yew_bootstrap::util::*;
use yew_bootstrap::component::{Row, Column};
use yewdux::prelude::{use_store, Dispatch};
use crate::components::common::main_menu::MainMenu;
use crate::routes::{Route, switch};
use crate::stores::user::UserState;
use yew::{function_component, html, Html, ContextProvider, Properties};

#[derive(Clone, Default, Properties, PartialEq)]
pub struct UserContext {
    state: UserState,
    dispatch: Dispatch<UserState>
}


#[function_component(App)]
fn app() -> Html {
    let (state, dispatch) = use_store::<UserState>();
    let user_context = UserContext {state: (*state).clone(), dispatch: dispatch.clone()};    
    html! {
        <>  
            {include_inline()}
            {include_cdn_icons()}
            <ContextProvider<UserContext> context={user_context}>
                <BrowserRouter>
                    <Row>
                    <Column class="bg-primary">
                        <MainMenu />
                    </Column>
                    </Row>
                    <Row>
                    <Column>
                        <Switch<Route> render={switch} /> 
                    </Column>
                    </Row>
                </BrowserRouter>
            </ContextProvider<UserContext>>
            { include_cdn_js() }
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}