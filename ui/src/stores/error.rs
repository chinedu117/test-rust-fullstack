use bevy_reflect::Uuid;
use yewdux::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Default)]
pub enum ErrorTypes {
    #[default]
    ApiError,
    AuthError
}

#[derive(Default, PartialEq, Serialize, Deserialize, Store, Clone)]
pub struct ErrorState {
    pub id: Uuid,    
    pub error: String,
    pub ty: ErrorTypes
}

#[derive(Default, PartialEq, Serialize, Deserialize, Store, Clone)]
pub struct ErrorStateList {
    pub errors: Vec<ErrorState>
}

pub fn error_type_title(err: ErrorTypes) -> String {
    match err {
        ErrorTypes::ApiError => "Error requesting from the API".to_string(),
        ErrorTypes::AuthError => "Authentication Error".to_string(),
    }
}