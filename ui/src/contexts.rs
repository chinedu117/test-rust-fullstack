use yewdux::prelude::Dispatch;
use yew::Properties;

use crate::stores::{user::UserState, error::ErrorStateList};

#[derive(Clone, Default, Properties, PartialEq)]
pub struct UserContext {
    pub state: UserState,
    pub dispatch: Dispatch<UserState>
}


#[derive(Clone, Default, Properties, PartialEq)]
pub struct ErrorContext {
    pub states: ErrorStateList,
    pub dispatch: Dispatch<ErrorStateList>
}