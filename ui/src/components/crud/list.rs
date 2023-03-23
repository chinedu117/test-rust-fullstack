use std::{rc::Rc};
use bevy_reflect::{Struct, Uuid};
use serde::{Deserialize, Serialize};
use yew::{prelude::*};
use gloo_net::{http::{Method::GET}, Error};
use yewdux::prelude::Dispatch;
use crate::{api::Client, UserContext, ErrorContext, stores::{error::{ErrorState, ErrorTypes, ErrorStateList}, user::UserState}};
use super::table::CrudTable;
use gloo_console::log;

#[derive(Properties, PartialEq, Clone)]
pub struct ItensProps {
    pub url: String,
    pub title: String

}

async fn api_request<M>(state: UserState, dispatch_user: Dispatch<UserState>, url: String) -> Result<Vec<M>, Error> where 
    M: for<'a> Deserialize<'a> {
    let client = Client {url: url, method: GET, body: None,
        state: state, dispatch: dispatch_user };
    let resp = client.send_request().await;
    log!("Requesting API...");
    match resp {
        Ok(res) => {
            let fetched: Result<Vec<M>, Error> = res.json().await;
            fetched            
        }
        Err(err) => {                               
           Err(err)
        }
    }  
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
                    let resp = api_request::<M>(user_context.state.clone(), 
                        user_context.dispatch.clone(), props.url.clone()).await;
                    match resp {
                        Ok(fetched) => { set_all.emit(fetched) }
                        Err(err) => {        
                            let dispatch_error = error_context.dispatch.clone();
                            let error_cb: Callback<ErrorStateList> = dispatch_error.reduce_callback(move |state: Rc<ErrorStateList>| {
                                let mut old_list = state.errors.clone();
                                old_list.push(ErrorState { error: err.to_string(), ty: ErrorTypes::ApiError, id: Uuid::new_v4() });
                                Rc::new(ErrorStateList { errors: old_list })
                            });
                            error_cb.emit(Default::default())
                        }
                    }    
                });
                || ()
            }, ());
        }        
        html! {
            <>
                <h3>{props.title.clone()}</h3>
                <hr />
                <CrudTable<M> list={all.clone().to_vec()} />                
            </>
        }
}
