

use yew::{html, Html, Properties, function_component, Callback, use_state};
use gloo_console::log;


#[derive(Properties, PartialEq, Clone, Default)]
pub struct ErrorToastProps {
    pub error: String
}

impl ErrorToastProps {
    fn show_class(&self) -> String{
        match self.error.is_empty() {
            true => "toast fade hide".to_string(),
            false => "toast fade show".to_string()
        }
    }
}

#[function_component(ErrorToast)]
pub fn error_toast(props: &ErrorToastProps) -> Html {
    log!("Props is empty: ", props.error.is_empty());
    log!("Props error: ", props.error.clone());
    let close = use_state(|| false);
    let cloned_close = close.clone();
    let cb_close = Callback::from(move |_| {
        cloned_close.set(true);
    });
    html! {
        if !props.error.is_empty() && !*close {
            <div aria-live={"polite"} aria-atomic={"true"} style={"position: relative; min-height: 200px;"}>
                <div style="position: absolute; top: 0; right: 0;">
                    
                    <div class={props.show_class()} role={"alert"} aria-live={"assertive"} aria-atomic={"true"} id={"error_toast"}>
                        <div class={"toast-header"}>
                            <strong class="mr-auto">{"Api Error"}</strong>
                            <small>
                                <button type={"button"}
                                    class={"btn-close ml-2 mb-1 close"}
                                    aria-label={"Close"}
                                    data-dismiss="toast"
                                    onclick={cb_close}>
                                    
                            </button>
                            </small>
                        </div>
                        <div class={"toast-body"}>
                            <strong class={"mr-auto"}>{props.error.clone()}</strong>
                        </div>
                    </div>
                
                </div>            
            </div>
        }
    }
}