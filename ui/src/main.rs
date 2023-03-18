mod components;
mod stores;
mod api;
mod routes;
use yew_router::prelude::*;
use yew_bootstrap::util::*;
use yew_bootstrap::component::{Row, Column};
use yewdux::prelude::{use_store, Dispatch};
use crate::components::common::error_toast::ErrorToast;
use crate::components::common::main_menu::MainMenu;
use crate::routes::{Route, switch};
use crate::stores::user::{UserState};
use crate::stores::error::{ErrorState};
use yew::{function_component, html, Html, ContextProvider, Properties};

#[derive(Clone, Default, Properties, PartialEq)]
pub struct UserContext {
    state: UserState,
    dispatch: Dispatch<UserState>
}

#[derive(Clone, Default, Properties, PartialEq)]
pub struct ErrorContext {
    state: ErrorState,
    dispatch: Dispatch<ErrorState>
}


#[function_component(App)]
fn app() -> Html {
    let (user_state, user_dispatch) = use_store::<UserState>();
    let (error_state, error_dispatch) = use_store::<ErrorState>();
    
    let user_context = UserContext {state: (*user_state).clone(), dispatch: user_dispatch.clone()};    
    let error_context = ErrorContext {state: (*error_state).clone(), dispatch: error_dispatch.clone()};    
    html! {
        <>  
            {include_inline()}
            {include_cdn_icons()}
            <ContextProvider<UserContext> context={user_context}>
                <ContextProvider<ErrorContext> context={error_context}>
                    <BrowserRouter>
                        <Row>
                        <Column class="bg-primary">
                            <MainMenu />                            
                        </Column>
                        </Row>
                        <Row>
                        <Column>
                            <ErrorToast />
                            <Switch<Route> render={switch} />                             
                        </Column>
                        </Row>
                    </BrowserRouter>
                </ContextProvider<ErrorContext>>
            </ContextProvider<UserContext>>
            { include_cdn_js() }
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}