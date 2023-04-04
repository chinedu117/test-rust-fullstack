use crate::components::form::{FormField, FormModel, FormType};
use shared_models::user::UserUi;


impl FormModel<UserUi> for UserUi {
    fn fields(&self) -> Vec<FormField<UserUi>> {
        vec![
            FormField {
                name: "username".to_string(),
                title: "Email Address".to_string(),
                ty: FormType::Input,
                parent: None,
            },
            FormField {
                name: "service".to_string(),
                title: "Auth Provider".to_string(),
                ty: FormType::Input,
                parent: None,
            }
        ]
    }

    fn title(&self) -> String {
        self.username.clone()
    }

    fn get_id(&self) -> i32 {
        self.id.clone()
    }
}

