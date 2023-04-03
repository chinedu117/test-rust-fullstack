use leptos::*;
use leptos_router::*;
use crate::{services::localstorage::{UserContext, UserState}, UserStateSetter, MessageStateSetter, components::messages::{MessageContext, Message, MessageType}};


#[component]
pub fn Auth(cx: Scope) -> impl IntoView {
    let user_state = use_context::<ReadSignal<UserContext>>(cx).clone().unwrap();
    let user_state_setter = use_context::<UserStateSetter>(cx).unwrap().0;
    let params = use_params_map(cx);
    let token = params.with(|p| p.get("token").cloned().unwrap_or_default());
    let current_state = UserState {logged: true, token: Some(token.clone())};    
    let mut old_ctx = user_state.get();
    match old_ctx.set_state(current_state.clone()) {
        Ok(_) => {
            user_state_setter.set(UserContext::new());
            return view! {cx,
                <div class="col-12">
                    <h1>"Authenticated!"</h1>
                    <Redirect path="/" />
                </div>
            }
        },
        Err(e) => {
            let message_state = use_context::<ReadSignal<MessageContext>>(cx).clone().unwrap();
            let message_state_setter = use_context::<MessageStateSetter>(cx).unwrap().0;
            let new_message = Message::new(e.to_string(), MessageType::AuthError);
            message_state.get().add(new_message, message_state_setter.clone());    
            return view! {cx,
                <div class="col-12">
                    <h1>"Error authenticating!"</h1>
                    <p>{e.to_string()}</p>
                    <Redirect path="/" />
                </div>
            }
        }
    }    
}