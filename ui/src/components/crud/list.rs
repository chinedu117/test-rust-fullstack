use bevy_reflect::{Struct};
use serde::{Deserialize, Serialize};
use yew::{prelude::*};
use yew_bootstrap::component::{Line};
use gloo_net::{http::{Method::GET}};
use crate::{api::Client, components::common::error::{ErrorToast}, UserContext};
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
        let request_error = use_state(|| "".to_string());
        let request_error_clone = request_error.clone();
        let set_error = Callback::from(move |request_error| request_error_clone.set(request_error)); 
        let user_context = use_context::<UserContext>().expect("no ctx found for UserContext");             
        {    
            let set_all = set_all.clone();            
            let set_error = set_error.clone();                            
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
                            set_error.emit(err.to_string());                            
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
                <table class={"table table-dark"}><CrudTable<M> list={all.clone().to_vec()} /></table>
                <ErrorToast error={request_error.clone().to_string()} />   
            </>
        }
}



