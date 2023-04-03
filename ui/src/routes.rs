use leptos::*;
use leptos_router::*;
use shared_models::organization::OrganizationUi;
use shared_models::user::UserUi;

use crate::components::home::{Home, HomeProps};
use crate::components::crud::{Crud, CrudProps};
use crate::components::auth::{Auth, AuthProps};
use crate::components::logout::{Logout, LogoutProps};
use crate::components::login::{Login, LoginProps};

#[component]
pub fn AppRouter(cx: Scope) -> impl IntoView {            
    view! { cx,
        <main>
            <Routes>
                <Route
                    path=""
                    view=move |cx| view! { cx,  <Home /> }
                />
                    
                <Route
                    path="users"
                    view=move |cx| view! { cx,  
                        <Crud _model={UserUi::default()} 
                            url={"/users/".to_string()} 
                            title={"Users".to_string()} /> 
                    }
                /> 

                <Route
                    path="organizations"
                    view=move |cx| view! { cx,  
                        <Crud _model={OrganizationUi::default()} 
                            url={"/organizations/".to_string()} 
                            title={"Organizations".to_string()} /> 
                    }
                /> 

                <Route
                    path="auth/:token"
                    view=move |cx| view! { cx,  <Auth /> }
                />
                <Route
                    path="logout"
                    view=move |cx| view! { cx,  <Logout /> }
                />
                <Route
                    path="login"
                    view=move |cx| view! { cx,  <Login /> }
                />                    
            </Routes>
        </main>
    }
}