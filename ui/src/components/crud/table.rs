use std::collections::HashMap;
use serde::{Serialize};
use serde_json::{Value};
use yew::{prelude::*};
use gloo_console::log;



#[derive(Properties, PartialEq, Clone)]
pub struct ListProps<M: Properties + PartialEq> {
        pub list: Vec<M>

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
                html! {
                    <tr>
                        {fields}                        
                    </tr>
                }
            })
            .collect::<Html>();
        html! {
            <table class={"table table-dark"}>
                {rows.clone()}
            </table>
        }                
    }