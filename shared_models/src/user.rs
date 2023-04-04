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

