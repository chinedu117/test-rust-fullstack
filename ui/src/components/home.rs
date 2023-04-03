use leptos::*;
use crate::services::localstorage::UserContext;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    let user_state = use_context::<ReadSignal<UserContext>>(cx).clone().unwrap();
    view! {cx,
        <div class="col-12">
            <h1>"Home"</h1>
            <p>{format!("Logged: {}", user_state.get().state.logged)}</p>
        </div>
    }
}