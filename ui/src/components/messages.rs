use leptos::*;
use rand::Rng;
use crate::jsbinds::toast;

use crate::MessageStateSetter;


#[derive(Clone, Default, PartialEq, Eq)]
pub enum MessageType {
    #[default]
    ApiError,
    AuthError,
    Accepted,
    NotFound
}

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Message {
    pub id: u32,
    pub message: String,
    pub message_type: MessageType,
}

impl Message {
    pub fn new(message: String, message_type: MessageType) -> Self {
        let mut rng = rand::thread_rng();
        let id: u32 = rng.gen();
        Self {
            id,
            message,
            message_type,
        }
    }
}


#[derive(Clone, Default, PartialEq, Eq)]
pub struct MessageContext {
    pub messages: Vec<Message>,
}

impl MessageContext {
    pub fn add(&mut self, message: Message, setter: WriteSignal<MessageContext>) {
        let id = message.id.to_string().clone();        
        self.messages.push(message);
        setter.set(self.clone());        
        toast::autohide(&id);

    }
    pub fn remove(&mut self, id: u32, setter: WriteSignal<MessageContext>) {
        let filtered = self.messages.iter().filter(|msg| msg.id != id).cloned().collect::<Vec<Message>>();
        setter.set(MessageContext { messages: filtered });
    }
}

fn message_class(message_type: MessageType) -> String {
    match message_type {
        MessageType::ApiError => "bg-danger text-white",
        MessageType::AuthError => "bg-warning text-white",
        MessageType::NotFound => "bg-warning text-white",
        MessageType::Accepted => "bg-success",
    }.to_string()
}

fn message_title(message_type: MessageType) -> String {
    match message_type {
        MessageType::ApiError => "Error from API",
        MessageType::AuthError => "Authentication Error",
        MessageType::NotFound => "Not Found",
        MessageType::Accepted => "Success",
    }.to_string()
}

#[component]
pub fn MessageContainer(cx: Scope) -> impl IntoView {
let message_state = use_context::<ReadSignal<MessageContext>>(cx).clone().unwrap();
let message_state_setter = use_context::<MessageStateSetter>(cx).unwrap().0;
view!{cx,
    <div aria-live="polite" aria-atomic="true" class="position-relative">
        <div class="toast-container position-absolute top-0 end-0 p-3">
            <div style="display: none">{message_state.get().messages.len()}</div>
            <For 
                each=move || message_state.get().messages
                key=|message| message.id 
                view=move |cx, message: Message| {
                    view! {cx,
                        <div class="toast" id={message.id.to_string()} role="alert" aria-live="assertive" aria-atomic="true">
                            <div class={format!("toast-header {}", message_class(message.message_type.clone()))}>
                            <strong class="mr-auto">{message_title(message.message_type.clone())}</strong>                             
                            <button type="button" class="btn-close bg-white" aria-label="Close" on:click=move |_| {
                                    let mut ctx = message_state.get();
                                    ctx.remove(message.id, message_state_setter);
                            }/>                            
                            </div>
                            <div class="toast-body">
                                {message.message}
                            </div>
                        </div>
                    }
            } />
        </div>
    </div>
}

}