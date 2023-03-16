use bevy_reflect::Reflect;
use yew::prelude::*;
use serde::{Deserialize, Serialize};
use shared_models::user::UserModel;


macro_rules! add_fields {
    ($name:ident, {}) => {
        #[derive(Clone, PartialEq, Deserialize, Reflect, Properties, Serialize, Default)]
        pub struct UiUser {
            $(
                pub $field:ident: $ty:ty,
                pub [<__gen_$field>]: Option<$ty>
            ),*
        }
        
        impl UiUser {
            pub fn set_fields(&mut self, fields: &[(&str, Option<String>)]) {
                for (name, value) in fields {
                    match name {
                        $(
                            stringify!($field) => {
                                if let Some(value) = value {
                                    self.$field = value.parse().unwrap();
                                } else {
                                    self.$field = self.[<__gen_$field>].take().unwrap();
                                }
                            }
                        )*
                        _ => {}
                    }
                }
            }
        }
    };
}



add_fields! (UserModel, {});

