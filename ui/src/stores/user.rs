use yewdux::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Default, PartialEq, Serialize, Deserialize, Store, Clone)]
#[store(storage = "local")]
pub struct UserState {
    pub logged: bool,
    pub token: String
}

