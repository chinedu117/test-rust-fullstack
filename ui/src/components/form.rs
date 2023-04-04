use std::fmt::Debug;
use std::hash::Hash;
use std::sync::Arc;
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_json::{Value};
use wasm_bindgen::JsValue;
use crate::components::messages::{Message, MessageContext, MessageType};
use crate::jsbinds::modal::closeCrudFormModal;
use crate::MessageStateSetter;
use crate::services::api::Client;

pub trait FormModel<M> {
    fn fields(&self) -> Vec<FormField<M>>;
    fn title(&self) -> String;
    fn get_id(&self) -> i32;
}

#[derive(Clone, PartialEq, Debug, Hash, Eq)]
pub enum FormType {
    Input,
    Select,
}

#[derive(Clone, PartialEq, Debug, Hash, Eq)]
pub struct FormField<M> {
    pub name: String,
    pub title: String,
    pub ty: FormType,
    pub parent: Option<M>
}

impl<M> FormField<M> where M: Serialize + for<'a> Deserialize<'a> + Clone + Debug + 'static + PartialEq + FormModel<M> + Hash + Eq {
    pub fn into_html(&self, cx: Scope, set_model: WriteSignal<Value>, model: ReadSignal<Value>, url: String, fetch: ReadSignal<bool>) -> impl IntoView {
        let name = self.name.clone();
        let ty = self.ty.clone();        
        let field = match ty {
            FormType::Input => {
                view! {cx,
                    <div>
                        <input type={name.clone()}
                            class="form-control"
                            on:input=move |ev| {
                                let new_value = event_target_value(&ev);
                                let mut new_model = model.get().clone();
                                new_model[name.clone()] = Value::String(new_value);
                                log!("set model: {:?}", model.get());
                                set_model.set(new_model);
                            }
                            prop:value=model.get()[self.name.clone()].as_str().unwrap_or_default()
                        />
                    </div>
                }
            }
            FormType::Select => {
                let (resource_list, set_resource_list) = create_signal::<Vec<M>>(cx, vec![]);
                create_resource(cx, move || fetch.get(), move |value| {
                    let cloned_url = url.clone();
                    async move {
                        if value == false {
                            return;
                        }
                        let result = Client::fetch_all::<M>("".to_string(), &cloned_url.clone()).await;
                        match result {
                            Ok(v) => {
                                log!("Request Ok...");
                                set_resource_list.set(v);
                            },
                            Err(msg) => {
                                log!("Failed to load API");
                                let message_state = use_context::<ReadSignal<MessageContext>>(cx).clone().unwrap();
                                let message_state_setter = use_context::<MessageStateSetter>(cx).unwrap().0;
                                message_state.get().add(msg, message_state_setter.clone());
                            }
                        }
                    }
                });
                view! {cx,
                    <div>
                        <select class="form-select"
                            on:input=move |ev| {
                                let new_value = event_target_value(&ev);
                                let mut new_model = model.get().clone();
                                new_model[name.clone()] = Value::Number(new_value.parse().unwrap());
                                log!("set model: {:?}", model.get());
                                set_model.set(new_model);
                            }
                        >
                        <option>"None"</option>
                            <For
                            each=move || resource_list.get()
                            key= |item| item.clone()
                            view=move |cx, item: M| {
                                view! {cx,
                                    <option value={item.get_id()}>{item.title()}</option>
                                }
                            } />
                        </select>
                    </div>
                }
            }
        };

        view! {cx,
            <label for={self.name.clone()} class="form-label">{self.title.clone()}</label>
            {field}
        }
    }
}

#[component]
pub fn CrudForm<M>(cx: Scope, selected: ReadSignal<Option<i32>>, set_fecth: WriteSignal<bool>, model: M, url: String, fetch: ReadSignal<bool> ) -> impl IntoView
    where M: Serialize + for<'a> Deserialize<'a> + Clone + Debug + 'static + PartialEq + FormModel<M> + Hash + Eq + Default {
    let (send, set_send) = create_signal(cx, false);
    let (form_model, set_form_model) = create_signal(cx, Value::default());
    let cloned_url = url.clone();
    let cloned_url2 = url.clone();

    create_resource(cx, move || selected.get(), move |value| {
        let cloned_url = Arc::new(url.clone());
        async move {
            if let Some(id) = value {
                let result = Client::fetch_one::<M>(id, &cloned_url).await;
                match result {
                    Ok(v) => {
                        let json = serde_json::to_string(&v).unwrap_or_default();
                        let generic: Value = serde_json::from_str(&json).unwrap_or_default();
                        set_form_model.set(generic);
                    },
                    Err(msg) => {
                        log!("Failed to load API");
                        let message_state = use_context::<ReadSignal<MessageContext>>(cx).clone().unwrap();
                        let message_state_setter = use_context::<MessageStateSetter>(cx).unwrap().0;
                        message_state.get().add(msg, message_state_setter.clone());
                    }
                }
            }
        }
    });

    create_resource(cx, move || send.get(), move |value| {
        log!("Send received {:?}", value);
        let cloned_url = cloned_url.clone();
        async move {
            if value {
                log!("Sending Request using {:?}", form_model.get());
                let message_state = use_context::<ReadSignal<MessageContext>>(cx).clone().unwrap();
                let message_state_setter = use_context::<MessageStateSetter>(cx).unwrap().0;
                let result = Client::create_one(&cloned_url,
                                                                  JsValue::from_str(&form_model.get().to_string())).await;
                match result {
                    Ok(_) => {
                        let msg = Message::new("Created successfully".to_string(), MessageType::Accepted);
                        message_state.get().add(msg, message_state_setter.clone());
                        closeCrudFormModal();
                        set_form_model.set(Value::default());
                        log!("Reseting Form: {:?}", form_model.get());                        
                        set_fecth.set(true);
                    },
                    Err(msg) => {
                        message_state.get().add(msg, message_state_setter.clone());
                    }
                }
                set_send.set(false);
            }
        }
    });

    view! {cx,
        <form id="crudFormFields">            
            <For
                each=move || model.fields()
                key= |field| field.name.clone()
                view=move |cx, field| {
                    log!("Printing Form Fields...");
                    view! {cx,
                    <div class="modal-body">
                            <div class="mb-3">
                                {field.into_html(cx, set_form_model.clone(), form_model.clone(), cloned_url2.clone(), fetch.clone())}
                            </div>
                    </div>
                    }
            } />        
            <div class="modal-footer">
                <button type="button" class="btn btn-secondary" data-bs-dismiss="modal" id="closeCrudForm">"Close"</button>
                <button type="button" class="btn btn-primary" on:click=move |_| {
                    set_send.set(true);
                }>"Save"</button>
            </div>
        </form>
    }
}