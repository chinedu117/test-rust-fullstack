use leptos::*;
use leptos_router::*;

use crate::services::localstorage::UserContext;

#[component]
pub fn Navbar(cx: Scope) -> impl IntoView {
    let user_state = use_context::<ReadSignal<UserContext>>(cx).clone().unwrap();
    view! {cx,
        <nav class="navbar navbar-expand-lg bg-body-tertiary">
            <div class="container-fluid">
              <a class="navbar-brand" href="#">"Ui"</a>
              <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
                <span class="navbar-toggler-icon"></span>
              </button>
              <div class="collapse navbar-collapse" id="navbarSupportedContent">
                <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                  <li class="nav-item">
                    <A exact=true href="/" class="nav-link">"Home"</A>
                  </li>
                  <Show 
                  when=move || user_state.get().state.logged 
                  fallback=|_| view! { cx, <></> }
                  >
                    <li class="nav-item">
                      <A exact=true href="/users" class="nav-link">"Users"</A>
                    </li>
                    <li class="nav-item">
                      <A exact=true href="/organizations" class="nav-link">"Organizations"</A>
                    </li>
                    <li class="nav-item">
                      <A exact=true href="/logout" class="nav-link">"Logout"</A>
                    </li>
                  </Show>
                  <Show 
                  when=move || !user_state.get().state.logged 
                  fallback=|_| view! { cx, <></> }
                  >
                    <li class="nav-item">
                      <A exact=true href="/login" class="nav-link">"Login"</A>
                  </li>
                  </Show>
                </ul>                    
              </div>
            </div>
          </nav>
    }
}