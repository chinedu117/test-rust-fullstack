use std::collections::HashMap;
use serde::{Serialize};
use serde_json::{Value};
use yew::{prelude::*};
use gloo_console::log;
use ybc::TileCtx::{Parent};
use ybc::TileSize::Twelve;


#[derive(Properties, PartialEq, Clone)]
pub struct ListProps<M: Properties + PartialEq> {
        pub list: Vec<M>,
        pub delete_cb: Callback<String>

}

#[function_component(CrudTable)]
pub fn table<M>(props: &ListProps<M>) -> Html
    where M: Properties + Serialize + Clone {        
        log!("Table");
        let field_names = match props.list.get(0) {
            Some(f) => {
                let json = serde_json::to_string(f).unwrap();
                let generic: HashMap<String, Value> =
                    serde_json::from_str(&json).unwrap();
                let mut names = generic.keys().cloned().collect::<Vec<String>>();
                names.sort(); 
                names
            },
            None => Vec::<String>::with_capacity(0)
        };
        
        let rows = props.list
            .iter()
            .map(|item| {
                let json = serde_json::to_string(item).unwrap();
                let generic: HashMap<String, Value> =
                    serde_json::from_str(&json).unwrap();                
                let fields: Html = field_names
                    .iter()
                    .map(|field_name| {
                        let field_name_rendered = field_name.clone();
                        html_nested! {
                            <td> 
                                <p>{&generic[&field_name_rendered]}</p>
                            </td>
                        }
                    })
                    .collect();
                let delete_cb = props.delete_cb.clone();
                html! {
                    <tr>
                        {fields}
                        <td>
                            <ybc::Dropdown>
                                <a onclick={move |_| delete_cb.emit(generic[&"id".to_string()].to_string()) }>{"Delete"}</a>
                            </ybc::Dropdown>
                        </td>                                        
                    </tr>
                }
            })
            .collect::<Html>();
        html! {
            <ybc::Tile ctx={Parent} vertical=true size={Twelve}>
                    <table class={"table is-bordered"}>
                        {rows.clone()}
                    </table>
            </ybc::Tile> 
        }                
    }