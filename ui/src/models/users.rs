use bevy_reflect::Reflect;
use yew::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Clone, PartialEq, Deserialize, Reflect, Properties, Serialize, Default)]
pub struct UserModel {
    id: i32,
    username: String,
    service: String
}

