use serde_json::json;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ToastOptions {
    pub autohide: bool,
    pub delay: u32,
    pub animation: bool,
}


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen]
    pub fn autohide(id: &str);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(module = Toast)]
    pub fn toast(id: String);
}



