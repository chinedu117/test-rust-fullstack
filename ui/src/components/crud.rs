use std::{collections::HashMap, fmt::Debug, sync::Arc};
use std::hash::Hash;
use leptos::*;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use crate::{
    services::{api::Client}, 
    components::{table::{CrudTable, CrudTableProps}, form::{FormModel, FormField, FormType, CrudForm, CrudFormProps},
    messages::{Message, MessageContext, MessageType}, form_modal::{FormModal, FormModalProps}}, MessageStateSetter
};



fn into_jsvalue<M>(v: Vec<M>) -> (Vec<String>, Vec<HashMap<String, Value>>) where M: Serialize + for<'a> Deserialize<'a> + Clone + Debug + 'static + PartialEq {
    let json = serde_json::to_string(&v).unwrap_or_default();
    let generic: Vec<HashMap<String, Value>> =
        serde_json::from_str(&json).unwrap_or_default();
    let default_hashmap = HashMap::new();
    let first = generic.get(0).unwrap_or(&default_hashmap);
    let mut names = first.keys().cloned().collect::<Vec<String>>();
    names.sort();
    (names, generic)
}

#[component]
pub fn Crud<M>(cx: Scope, _model: M, url: String, title: String) -> impl IntoView 
    where M: Serialize + for<'a> Deserialize<'a> + Clone + Debug + 'static + PartialEq + FormModel<M> + Hash + Eq + 'static + Default {
    
    let (filter, _) = create_signal::<String>(cx, "".to_string());
    let (fetch, set_fetch) = create_signal::<bool>(cx, true);
    let (delete, set_delete) = create_signal::<Option<i32>>(cx, None);
    let (fields, fields_setter) = create_signal::<Vec<String>>(cx, vec![]);
    let (values, values_setter) = create_signal::<Vec<HashMap<String, Value>>>(cx, vec![]);
    let (selected, _) = create_signal::<Option<i32>>(cx, None);
    let cloned_url = url.clone();
    let cloned_url2 = url.clone();


    create_resource(cx, move || fetch.get(), move |value| {
        log!("Fetch received {:?}", value);
        let cloned_url = Arc::new(cloned_url.clone());
        async move {
            if value {
                log!("Sending Request...");
                let result = Client::fetch_all::<M>(filter.get(), &cloned_url).await;
                    match result {
                        Ok(v) => {                            
                            let (fields, values) = into_jsvalue(v);
                            fields_setter.set(fields.clone());
                            values_setter.set(values.clone());
                            log!("Current items: {}", values.len());
                        },
                        Err(msg) => {                            
                            let message_state = use_context::<ReadSignal<MessageContext>>(cx).clone().unwrap();
                            let message_state_setter = use_context::<MessageStateSetter>(cx).unwrap().0;
                            message_state.get().add(msg, message_state_setter.clone());                                            
                            fields_setter.set(Vec::new());
                            values_setter.set(Vec::new());
                        }
                    }
            }
            set_fetch.set(false);
        }
    });
    
    create_resource(cx, move || delete.get(), move |value| {        
        let cloned_url = Arc::new(cloned_url2.clone());
        async move {
            let message_state = use_context::<ReadSignal<MessageContext>>(cx).clone().unwrap();
            let message_state_setter = use_context::<MessageStateSetter>(cx).unwrap().0;
            match value {
                Some(id) => {
                    set_delete.set(None);
                    match Client::delete_one(&cloned_url, id).await {
                        Ok(_) => {
                            let msg = Message::new("Deleted successfully".to_string(), MessageType::Accepted);
                            message_state.get().add(msg, message_state_setter.clone());
                            set_fetch.set(true);
                        },
                        Err(msg) => {                                                        
                            message_state.get().add(msg, message_state_setter.clone());                                            
                        }
                    }
                },
                None => {}
            }
        }
    }); 

    view! {cx,        
        <div class="row">
            <div class="col">
                <h1>{title}</h1>
            </div>            
        </div>
        <div class="row">
            <div class="col">
                <div class="float-end">
                    <FormModal>
                        <CrudForm model={_model} set_fecth={set_fetch} selected={selected} url={url.clone()} fetch={fetch.clone()}/>                        
                    </FormModal>
                </div>
            </div>            
        </div>
        <div class="row">
            <div class="col">
                <Suspense
                    fallback=move || view! { cx, <p>"Loading..."</p> }
                >
                    <CrudTable fields={fields} rows={values} delete_cb={set_delete}/>            
                </Suspense>
            </div>
        </div>
       
    }
}