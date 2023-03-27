use yew_router::prelude::*;
use yewdux::prelude::{use_store};
use crate::components::common::messages::MessageContainer;
use crate::components::main_menu::MainMenu;
use crate::contexts::{UserContext, MessageContext};
use crate::routes::{Route, switch};
use crate::stores::messages::MessageStateList;
use crate::stores::user::{UserState};
use yew::{function_component, html, Html, ContextProvider, classes};
use ybc::TileCtx::{Child, Parent};
use ybc::TileSize::Twelve;


#[function_component(App)]
pub fn app() -> Html {
    let (user_state, user_dispatch) = use_store::<UserState>();
    let (error_state, error_dispatch) = use_store::<MessageStateList>();
    
    let user_context = UserContext {state: (*user_state).clone(), dispatch: user_dispatch.clone()};    
    let error_context = MessageContext {states: (*error_state).clone(), dispatch: error_dispatch.clone()};    
    html! {
        <>  
            <ContextProvider<UserContext> context={user_context}>
                <ContextProvider<MessageContext> context={error_context}>
                    <BrowserRouter>
                        
                        <MainMenu />
                        
                        <section class={"section"}>
                            <ybc::Container fluid=true>
                                <ybc::Tile ctx={Parent} vertical=true size={Twelve} classes={classes!("box")}>
                                <MessageContainer />
                                        
                                        <Switch<Route> render={switch} />
                                    
                                </ybc::Tile>        
                            </ybc::Container>
                        </section>
                    </BrowserRouter>
                </ContextProvider<MessageContext>>
            </ContextProvider<UserContext>>            
        </>
    }
}