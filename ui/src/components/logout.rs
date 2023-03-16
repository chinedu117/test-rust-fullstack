use yew::prelude::*;
use yewdux::prelude::*;
use crate::stores::user::UserState;
use yew_router::prelude::*;
use crate::Route;

#[function_component(Logout)]
pub fn logout() -> Html {
    let (_, dispatch) = use_store::<UserState>();
    let dispatch: Callback<UserState> = dispatch.reduce_mut_callback(move |state| {
        state.logged = false;
        state.token = "".to_string()
    });
    dispatch.emit(Default::default());
    html! {
        <Redirect<Route> to={Route::Home}/>
    }
}