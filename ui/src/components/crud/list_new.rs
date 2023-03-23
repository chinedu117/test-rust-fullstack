use std::{rc::Rc};
use bevy_reflect::{Struct, Uuid};
use serde::{Deserialize, Serialize};
use yew::{prelude::*};
use gloo_net::{http::{Method::GET, Method::DELETE}, Error};
use yewdux::prelude::Dispatch;
use crate::{api::Client, UserContext, ErrorContext, stores::{error::{ErrorState, ErrorTypes, ErrorStateList}, user::UserState}, components::crud::tableNew::CrudTableNew};
use super::table::CrudTable;
use ybc::TileCtx::{Child, Parent};
use ybc::TileSize::Twelve;

#[derive(Properties, PartialEq, Clone)]
pub struct ItensProps {
    pub url: String,
    pub title: String

}

fn delete_one_cb<M>(user_state: UserState, 
    user_dispatch: Dispatch<UserState>, 
        error_dispatch: Dispatch<ErrorStateList>, url: String) -> Callback<String> where
            M: Properties + PartialEq + Struct + for<'a> Deserialize<'a> + Clone + Serialize + Default { 
    Callback::from(move |id: String| {
        let user_state = user_state.clone();
        let user_dispatch = user_dispatch.clone();
        let error_dispatch = error_dispatch.clone();
        let url = url.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let user_state = user_state.clone();
            let client = Client {url: format!("{}/{}", url, id), method: DELETE, body: None,
                state: user_state, dispatch: user_dispatch };
            let resp = client.send_request().await;
            let dispatch_error = error_dispatch.clone();
            match resp {
                Ok(res) => {
                    let error_cb: Callback<ErrorStateList> = dispatch_error.reduce_callback(move |state: Rc<ErrorStateList>| {
                        let mut old_list = state.errors.clone();
                        old_list.push(ErrorState { error: res.status_text(), ty: ErrorTypes::ApiError, id: Uuid::new_v4() });
                        Rc::new(ErrorStateList { errors: old_list })
                    });
                    error_cb.emit(Default::default())
                }
                Err(err) => {        
                    let error_cb: Callback<ErrorStateList> = dispatch_error.reduce_callback(move |state: Rc<ErrorStateList>| {
                        let mut old_list = state.errors.clone();
                        old_list.push(ErrorState { error: err.to_string(), ty: ErrorTypes::ApiError, id: Uuid::new_v4() });
                        Rc::new(ErrorStateList { errors: old_list })
                    });
                    error_cb.emit(Default::default())
                }
            }    
        });
    })
}

fn fetch_all_cb<M>(user_state: UserState, 
    user_dispatch: Dispatch<UserState>, 
        error_dispatch: Dispatch<ErrorStateList>, set_all: Callback<Vec<M>>, url: String) -> Callback<()> where
            M: Properties + PartialEq + Struct + for<'a> Deserialize<'a> + Clone + Serialize + Default { 
    Callback::from(move |_:_| {
        let user_state = user_state.clone();
        let user_dispatch = user_dispatch.clone();
        let error_dispatch = error_dispatch.clone();
        let set_all = set_all.clone();            
        let url = url.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let user_state = user_state.clone();
            let client = Client {url: url, method: GET, body: None,
                state: user_state, dispatch: user_dispatch };
            let resp = client.send_request().await;
            match resp {
                Ok(res) => {
                    let fetched: Result<Vec<M>, Error> = res.json().await; 
                    set_all.emit(fetched.unwrap()) 
                }
                Err(err) => {        
                    let dispatch_error = error_dispatch.clone();
                    let error_cb: Callback<ErrorStateList> = dispatch_error.reduce_callback(move |state: Rc<ErrorStateList>| {
                        let mut old_list = state.errors.clone();
                        old_list.push(ErrorState { error: err.to_string(), ty: ErrorTypes::ApiError, id: Uuid::new_v4() });
                        Rc::new(ErrorStateList { errors: old_list })
                    });
                    error_cb.emit(Default::default())
                }
            }    
        });
    })
}


#[function_component(CrudListNew)]
pub fn list<M>(props: &ItensProps) -> Html where 
    M: Properties + PartialEq + Struct + for<'a> Deserialize<'a> + Clone + Serialize + Default {        
        let all: UseStateHandle<Vec<M>> = use_state(|| vec![]);
        let all_clone = all.clone();
        let set_all = Callback::from(move |list| all_clone.set(list));        
        let user_context = use_context::<UserContext>().expect("no ctx found for UserContext");
        let error_context = use_context::<ErrorContext>().expect("no ctx found for UserContext");
        let error_dispatch = error_context.dispatch.clone();
        let user_state = user_context.state.clone();
        let user_dispatch = user_context.dispatch.clone();
        let url = props.url.clone();
        let fetch_all = fetch_all_cb::<M>(user_state.clone(), user_dispatch.clone(), 
            error_dispatch.clone(), set_all, url.clone());
        use_effect_with_deps(move |_| { 
            fetch_all.emit(Default::default())
        }, ());
        
        html! {
            <ybc::Tile ctx={Parent} vertical=true size={Twelve}>
                <h1 class={"title algolia-lvl0"}>{props.title.clone()}</h1>
                <hr />
                <CrudTableNew<M> list={all.clone().to_vec()} delete_cb={delete_one_cb::<M>(user_state.clone(), 
                    user_dispatch.clone(), error_dispatch, url)}/>                
            </ybc::Tile>
        }
}
