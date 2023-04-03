use leptos::*;
use leptos_router::*;

mod components;
mod services;
mod routes;
mod jsbinds;
mod forms;

use routes::{AppRouter, AppRouterProps};
use components::navbar::{Navbar, NavbarProps};
use components::messages::{MessageContainer, MessageContainerProps};
use services::localstorage::UserContext;
use crate::components::messages::MessageContext;


#[derive(Copy, Clone)]
struct UserStateSetter(WriteSignal<UserContext>);

#[derive(Copy, Clone)]
struct MessageStateSetter(WriteSignal<MessageContext>);

pub fn main() {    
    mount_to_body(|cx| {
        let (user_state, set_userstate) = create_signal::<UserContext>(cx, UserContext::new());
        let (message_state, set_messages) = create_signal::<MessageContext>(cx, MessageContext::default());
        provide_context(cx, user_state);
        provide_context(cx, UserStateSetter(set_userstate));
        provide_context(cx, message_state);
        provide_context(cx, MessageStateSetter(set_messages));
        view! { cx,
            <div class="container-fluid">
                <Router>
                    <div class="row">
                        <div class="col-12">
                            <Navbar />
                        </div>
                    </div>
                    <div class="row">
                        <MessageContainer />
                        <AppRouter />
                    </div>
                </Router>
            </div>
        }
    })
}