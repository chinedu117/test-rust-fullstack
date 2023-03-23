use yew_router::prelude::*;
use yewdux::prelude::{use_store};
use crate::components::common::error_toast::ErrorContainer;
use crate::components::common::main_menu::MainMenu;
use crate::contexts::{UserContext, ErrorContext};
use crate::routes::{Route, switch};
use crate::stores::error::ErrorStateList;
use crate::stores::user::{UserState};
use yew::{function_component, html, Html, ContextProvider, classes};
use ybc::TileCtx::{Child, Parent};
use ybc::TileSize::Twelve;


#[function_component(App)]
pub fn app() -> Html {
    let (user_state, user_dispatch) = use_store::<UserState>();
    let (error_state, error_dispatch) = use_store::<ErrorStateList>();
    
    let user_context = UserContext {state: (*user_state).clone(), dispatch: user_dispatch.clone()};    
    let error_context = ErrorContext {states: (*error_state).clone(), dispatch: error_dispatch.clone()};    
    html! {
        <>  
            <ContextProvider<UserContext> context={user_context}>
                <ContextProvider<ErrorContext> context={error_context}>
                    <BrowserRouter>
                        <MainMenu />
                         <ErrorContainer />
                         <ybc::Container fluid=true>
                            <ybc::Tile ctx={Parent} vertical=true size={Twelve}>
                                <ybc::Tile ctx={Child} classes={classes!("box")}>                            
                                    <Switch<Route> render={switch} />
                                </ybc::Tile>
                            </ybc::Tile>        
                         </ybc::Container>                                                     
                    </BrowserRouter>
                </ContextProvider<ErrorContext>>
            </ContextProvider<UserContext>>            
        </>
    }
}