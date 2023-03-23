use std::rc::Rc;
use bevy_reflect::Uuid;
use yew::{html, Html, function_component, Callback, use_context, Properties};
use crate::{ErrorContext, stores::error::{ErrorState, ErrorStateList, error_type_title}};
use gloo_console::log;

#[derive(Properties, PartialEq, Clone)]
pub struct ErrorProps {
    error: ErrorState,
    callback: Callback<Uuid>
}


#[function_component(ErrorContainer)]
pub fn error_container() -> Html {
    let error_context = use_context::<ErrorContext>().expect("no ctx found for UserContext");
    let error_dispatch = error_context.dispatch.clone();
    let states = error_context.states.clone();
    let onclick = Callback::from(move |id: Uuid| {
        log!("Received Id", id.to_string());        
        let states = states.clone();
        log!("Current State count", states.errors.len());
        let cb = error_dispatch.reduce_callback::<_, Rc<ErrorStateList>>(move |_| {
            let errors = states.errors.clone();
            let filtered: Vec<ErrorState> = errors.into_iter()                
                .filter(|e| { e.id != id})
                .collect();
            log!("State count after filtering", filtered.len());            
            Rc::new(ErrorStateList {errors: filtered.to_vec()})
        });      
        cb.emit(Default::default())  
    });
    html! {
        <div style="position: absolute; top: 130; right: 0;">
            {
                error_context.states.errors.to_vec().iter().map(|state| {
                    html! {
                        <ErrorToast error={state.clone()} callback={onclick.clone()} />
                    }
                }).collect::<Html>()
            }
        </div>            
    }
}


#[function_component(ErrorToast)]
fn error_toast(props: &ErrorProps) -> Html {
    let id = props.error.id.clone();
    let msg = props.error.error.clone();
    let ty = props.error.ty.clone();
    let title = error_type_title(ty);
    let onclick = props.callback.clone();
    html! {            
        <div class={"toast show"} style={"background-color: rgba(255,111,105,0.7) !important;"}
            role={"alert"} 
            aria-live={"assertive"} 
            aria-atomic={"true"} id={id.to_string()}>
            <div class={"toast-header"}>
                <small>
                    <strong class="mr-auto">
                        {title}
                    </strong>
                    <button type={"button"}
                        class={"btn-close ml-2 mb-1 close"}
                        aria-label={"Close"}
                        data-dismiss="toast" 
                        onclick={move |_|{ onclick.emit(id); }}>
                        
                </button>
                </small>
            </div>
            <div class={"toast-body"}>
                <strong class={"mr-auto"}>{msg}</strong>
            </div>
        </div>            
    }
}