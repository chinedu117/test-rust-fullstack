use yew::prelude::*;
use crate::{stores::user::UserState, UserContext};
use yew_router::prelude::*;
use crate::Route;

#[function_component(Logout)]
pub fn logout() -> Html {
    let user_context = use_context::<UserContext>().expect("no ctx found for UserContext");
    let user_dispatch = user_context.dispatch;    
    let user_dispatch: Callback<UserState> = user_dispatch.reduce_mut_callback(move |state| {
        state.logged = false;
        state.token = None;
    });
    user_dispatch.emit(Default::default());
    html! {
        <Redirect<Route> to={Route::Home}/>
    }
}