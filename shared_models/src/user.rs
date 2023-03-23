use std::collections::HashMap;

use macros::GenerateModels;
use serde::{Deserialize, Serialize};
use yew::Properties;
use bevy_reflect::Reflect;
use sea_orm::entity::prelude::*;
use sea_orm::{DeriveEntityModel, DeriveIntoActiveModel, EnumIter, DeriveRelation, ActiveModelBehavior};



#[derive(GenerateModels)]
struct User {
    id: i32,
    username: String,
    service: String
}


#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {} 

/*
impl FormFields for UserUi {
    fn names_and_types() ->  std::collections::HashMap<String, FormType> {
        let mut map: HashMap<String, FormType> = HashMap::new();
        map.insert("name".to_string(), FormType::Input);
        map.insert("service".to_string(), FormType::Input);
        map
    }
}  
*/