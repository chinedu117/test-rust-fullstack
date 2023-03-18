use std::{rc::Rc};
use bevy_reflect::{Struct};
use serde::{Deserialize, Serialize};
use yew::{prelude::*};
use yew_bootstrap::component::{Line};
use gloo_net::{http::{Method::GET}};
use crate::{api::Client, UserContext, ErrorContext, stores::error::{ErrorState, ErrorTypes}};
use super::table::CrudTable;
use gloo_console::log;

#[derive(Properties, PartialEq, Clone)]
pub struct ItensProps {
    pub url: String,
    pub title: String

}

#[function_component(CrudList)]
pub fn list<M>(props: &ItensProps) -> Html where 
    M: Properties + PartialEq + Struct + for<'a> Deserialize<'a> + Clone + Serialize + Default {
        let all: UseStateHandle<Vec<M>> = use_state(|| vec![]);
        let all_clone = all.clone();
        let set_all = Callback::from(move |list| all_clone.set(list));        
        let user_context = use_context::<UserContext>().expect("no ctx found for UserContext");
        let error_context = use_context::<ErrorContext>().expect("no ctx found for UserContext");             
        {    
            let set_all = set_all.clone();                                                    
            let props = props.clone();                        
            use_effect_with_deps(move |_| {                
                let props = props.clone();                
                wasm_bindgen_futures::spawn_local(async move {
                    let client = Client {url: props.url.clone(), method: GET, body: None,
                        state: user_context.state.clone(), dispatch: user_context.dispatch.clone() };
                    let resp = client.send_request().await;
                    log!("Requesting API...");
                    match resp {
                        Ok(res) => {
                            let fetched = res.json()
                                .await
                                .unwrap();
                            set_all.emit(fetched)
                        }
                        Err(err) => {                               
                            let s: Callback<ErrorState> = error_context.dispatch.reduce_callback(move |_| {
                                let state = ErrorState { error: Some(err.to_string()), ty: Some(ErrorTypes::ApiError) };
                                Rc::new(state)
                            });
                            s.emit(Default::default())

                        }
                    }    
                });
                || ()
            }, ());
        }        
        html! {
            <>
                <Line />
                <h3>{props.title.clone()}</h3>
                <Line />
                <CrudTable<M> list={all.clone().to_vec()} />                
            </>
        }
}



