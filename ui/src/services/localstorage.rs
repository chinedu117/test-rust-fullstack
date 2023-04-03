use gloo_storage::{LocalStorage, Storage, errors::StorageError};
use serde::{Deserialize, Serialize};

#[derive(Default, PartialEq, Serialize, Deserialize, Eq, Clone)]
pub struct UserState {
    pub logged: bool,
    pub token: Option<String>
}

#[derive(Clone, Default, PartialEq, Eq)]
pub struct UserContext {
    pub state: UserState
}

impl UserContext {
    pub fn new() -> Self {
        let state = LocalStorage::get::<UserState>("ui::stores::user::UserState").unwrap_or_default();
        Self { state }
    }

    pub fn set_state(&mut self, state: UserState) -> Result<(), StorageError> {
        self.state = state;
        LocalStorage::set("ui::stores::user::UserState", self.state.clone())
    }
}