use std::collections::HashMap;
use std::sync::Arc;

use leptos::*;
use serde_json::{Value, json};

#[component]
pub fn CrudTable(cx: Scope, fields: ReadSignal<Vec<String>>, 
    rows: ReadSignal<Vec<HashMap<String, Value>>>, delete_cb: WriteSignal<Option<i32>>) -> impl IntoView {
    let delete_cb_cloned = Arc::new(delete_cb.clone());
    view! {cx,
            <table class="table">
                <thead>
                    <tr>
                        <For 
                        each=move || fields.get()
                        key= |field| field.clone()
                        view=move |cx, field: String| {                            
                            view! {cx,
                                    <th>{field}</th>
                            }
                        } />                     
                    </tr>
                  </thead>
                <tbody>
                    <For 
                    each=move || rows.get()
                    key=|row| row.clone().get(&"id".to_string()).unwrap().to_string()
                    view=move |cx, row| { 
                        let delete_cb_cloned = Arc::clone(&delete_cb_cloned);
                        let id: i32 = row.clone().get(&"id".to_string()).unwrap().to_string().parse().unwrap();
                        view! {cx,
                            <tr>
                                <For
                                    each=move || fields.get()
                                    key= |field| field.clone()
                                    view=move |cx, field: String| {                                        
                                        view! {cx,
                                                  <td>{row.get(&field).unwrap_or(&json!("NaN")).to_string()}</td>
                                        }
                                    } />
                                    <td>
                                        <div class="dropdown">
                                            <button class="btn btn-sm btn-secondary dropdown-toggle" type="button" id="dropdownMenuButton" data-bs-toggle="dropdown" aria-haspopup="true" aria-expanded="false">
                                                "Menu"
                                            </button>
                                        
                                            <div class="dropdown-menu" aria-labelledby="dropdownMenuLink">
                                                <a class="dropdown-item" href="#" on:click={move |_| delete_cb_cloned.set(Some(id)) } >"Delete"</a>
                                            </div>
                                        </div>
                                    </td>
                            </tr>
                        }
                    } />
                </tbody>
            </table>                          
    }
}