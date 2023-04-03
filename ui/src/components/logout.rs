use leptos::*;
use leptos_router::*;
use crate::{services::localstorage::{UserContext, UserState}, UserStateSetter, components::messages::{MessageContext, MessageType, Message}, MessageStateSetter};


#[component]
pub fn Logout(cx: Scope) -> impl IntoView {
    let user_state = use_context::<ReadSignal<UserContext>>(cx).clone().unwrap();
    let user_state_setter = use_context::<UserStateSetter>(cx).unwrap().0;
    let current_state = UserState {logged: false, token: None};        
    match user_state.get().set_state(current_state.clone()) {
        Ok(_) => {
            log!("Logged out");
        },
        Err(err) => {
            log!("Error logging out: {:?}", err);
            let message_state = use_context::<ReadSignal<MessageContext>>(cx).clone().unwrap();
            let message_state_setter = use_context::<MessageStateSetter>(cx).unwrap().0;
            let new_message = Message::new(err.to_string(), MessageType::AuthError);
            message_state.get().add(new_message, message_state_setter.clone());
        }
    }
    user_state_setter.set(UserContext::new());
    view! {cx,
        <div class="col-12">
            <h1>"Logging out..."</h1>
            <Redirect path="/" />
        </div>
    }
}