use yewdux::prelude::Dispatch;
use yew::Properties;

use crate::stores::{user::UserState, messages::MessageStateList};

#[derive(Clone, Default, Properties, PartialEq)]
pub struct UserContext {
    pub state: UserState,
    pub dispatch: Dispatch<UserState>
}


#[derive(Clone, Default, Properties, PartialEq)]
pub struct MessageContext {
    pub states: MessageStateList,
    pub dispatch: Dispatch<MessageStateList>
}