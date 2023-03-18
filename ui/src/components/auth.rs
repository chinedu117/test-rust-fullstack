use yew::prelude::*;
use yewdux::prelude::*;
use crate::{stores::user::UserState, UserContext};
use yew_router::prelude::*;
use crate::Route;

#[derive(Clone, PartialEq, Properties)]
pub struct AuthProps {
    pub token: AttrValue
}

#[function_component(Auth)]
pub fn auth(props: &AuthProps) -> Html {
    let cloned_props = props.clone();
    let user_context = use_context::<UserContext>().expect("no ctx found for UserContext");
    let user_dispatch = user_context.dispatch;
    let user_state = user_context.state;        
    let user_dispatch: Callback<UserState> = user_dispatch.reduce_mut_callback(move |user_state| {
        let AuthProps { token } = cloned_props.clone();
        user_state.logged = true;
        user_state.token = token.to_string();        
    }); 
    user_dispatch.emit(Default::default());
    html! {
        <Redirect<Route> to={Route::Home}/>
    }
}