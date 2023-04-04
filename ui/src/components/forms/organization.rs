use shared_models::organization::OrganizationUi;
use crate::components::form::{FormField, FormModel, FormType};

impl FormModel<OrganizationUi> for OrganizationUi {
    fn fields(&self) -> Vec<FormField<OrganizationUi>> {
        vec![
            FormField {
                name: "name".to_string(),
                title: "Org Name".to_string(),
                ty: FormType::Input,
                parent: None,
            },
            FormField {
                name: "parent_id".to_string(),
                title: "Parent".to_string(),
                ty: FormType::Select,
                parent: Some(self.clone()),
            }
        ]
    }

    fn title(&self) -> String {
        self.name.clone()
    }

    fn get_id(&self) -> i32 {
        self.id.clone()
    }
}