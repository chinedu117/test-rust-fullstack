use std::collections::HashMap;


pub enum FormType {
    Input,
    Select,
    TextArea
}

pub trait FormFields {
    fn names_and_types() ->  HashMap<String, FormType>;
}