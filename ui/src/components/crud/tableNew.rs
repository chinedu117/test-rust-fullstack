use std::collections::HashMap;
use serde::{Serialize};
use serde_json::{Value};
use yew::{prelude::*};
use gloo_console::log;
use ybc::TileCtx::{Child, Parent};
use ybc::TileSize::Twelve;


#[derive(Properties, PartialEq, Clone)]
pub struct ListPropsNew<M: Properties + PartialEq> {
        pub list: Vec<M>,
        pub delete_cb: Callback<String>

}

#[function_component(CrudTableNew)]
pub fn table<M>(props: &ListPropsNew<M>) -> Html
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
                html! {
                    <tr>
                        {fields}
                        <td>
                            <div class={"dropdown"}>
                                <button class={"btn btn-secondary dropdown-toggle"} type={"button"} id={"dropdownMenuButton"} 
                                    data-toggle={"dropdown"} aria-haspopup={"true"} aria-expanded={"false"}>
                                    {"Dropdown button"}
                                </button>
                                <div class={"dropdown-menu"} aria-labelledby={"dropdownMenuButton"}>
                                    <a class={"dropdown-item"} href={"#"}>{"Action"}</a>
                                    <a class={"dropdown-item"} href={"#"}>{"Another action"}</a>
                                    <a class={"dropdown-item"} href={"#"}>{"Something else here"}</a>
                                </div>
                            </div>
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