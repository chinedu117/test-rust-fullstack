use std::collections::HashMap;

use macros::GenerateModels;
use serde::{Deserialize, Serialize};
use yew::Properties;
use bevy_reflect::Reflect;
use sea_orm::entity::prelude::*;
use sea_orm::{DeriveEntityModel, DeriveIntoActiveModel, EnumIter, DeriveRelation, ActiveModelBehavior};

use crate::traits::{FormFields, FormType};

#[derive(GenerateModels)]
struct Organization {
    id: i32,
    name: String,
    parent_id: Option<i32>
}


#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "Entity", from = "Column::ParentId", to = "Column::Id")]
    SelfReferencing,
}

impl ActiveModelBehavior for ActiveModel {} 

impl FormFields for OrganizationUi {
    fn names_and_types() ->  std::collections::HashMap<String, FormType> {
        let mut map: HashMap<String, FormType> = HashMap::new();
        map.insert("name".to_string(), FormType::Input);
        map.insert("service".to_string(), FormType::Select);
        map
    }
}    