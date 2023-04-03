use leptos::*;
use web_sys::window;

#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    let location = window()
                .expect("window is undefined")
                .location();
    match location.set_href("http://localhost:3000/auth/login") {
        Ok(_) => view! {cx,
            <div class="col-12">
                <h1>"Redirect to Login..."</h1>            
            </div>
        },
        Err(_) => view! {cx,
            <div class="col-12">
                <h1>"Cannot Redirect to Login Page..."</h1>            
            </div>
        },
    }        
    
}