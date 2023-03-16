use yew::prelude::*;
use yewdux::prelude::*;
use crate::stores::user::UserState;
use yew_router::prelude::*;
use crate::Route;

#[derive(Clone, PartialEq, Properties)]
pub struct AuthProps {
    pub token: AttrValue
}

#[function_component(Auth)]
pub fn auth(props: &AuthProps) -> Html {
    let cloned_props = props.clone();
    let (_, dispatch) = use_store::<UserState>();
    let dispatch: Callback<UserState> = dispatch.reduce_mut_callback(move |state| {
        let AuthProps { token } = cloned_props.clone();
        state.logged = true;
        state.token = token.to_string();
    }); 
    dispatch.emit(Default::default());
    html! {
        <Redirect<Route> to={Route::Home}/>
    }
}