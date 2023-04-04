use leptos::*;

#[component]
pub fn FormModal(cx: Scope, children: Children) -> impl IntoView {
    view! {cx,
        <button type="button" class="btn btn-primary" data-bs-toggle="modal" data-bs-target="#crudForm">
            "New"
        </button> 
        <div class="modal fade" id="crudForm" tabindex="-1" aria-labelledby="crudFormLabel" aria-hidden="true">
            <div class="modal-dialog">
            <div class="modal-content">
                <div class="modal-header">
                    <h1 class="modal-title fs-5" id="crudFormLabel">"New"</h1>
                </div>

                {children(cx)}

            </div>
            </div>
        </div>
    }
}