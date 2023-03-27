use super::table::CrudTable;
use crate::{
    components::crud::{
        callbacks::CrudCallBacks,
        form::{CrudForm, ToggleState}
    },
    MessageContext, UserContext,
};
use bevy_reflect::Struct;
use serde::{Deserialize, Serialize};
use ybc::TileSize::Twelve;
use yew::prelude::*;
use crate::components::forms::organization::OrganizationForm;
use yew::virtual_dom::VNode;

#[derive(Properties, PartialEq, Clone)]
pub struct ItensProps {
    pub url: String,
    pub title: String,
    pub form: Option<VNode>,
}

// fetch all itens from server and render a table with them
#[function_component(CrudList)]
pub fn list<M>(props: &ItensProps) -> Html
where
    M: Properties + PartialEq + Struct + for<'a> Deserialize<'a> + Clone + Serialize + Default,
{
    let all: UseStateHandle<Vec<M>> = use_state(|| vec![]);
    let toggle_form: UseStateHandle<ToggleState> = use_state(|| ToggleState::Hide);
    let toggle_form_cloned: UseStateHandle<ToggleState> = toggle_form.clone();
    let toggle_callback = Callback::from(move |new_state: ToggleState| toggle_form.set(new_state));
    let toggle_callback_cloned = toggle_callback.clone();
    let clone_all = all.clone();
    let set_all = Callback::from(move |list| all.set(list));
    let user_context = use_context::<UserContext>().expect("no ctx found for UserContext");
    let error_context = use_context::<MessageContext>().expect("no ctx found for UserContext");
    let error_dispatch = error_context.dispatch.clone();
    let user_state = user_context.state.clone();
    let user_dispatch = user_context.dispatch.clone();
    let url = props.url.clone();
    let callbacks = CrudCallBacks::<M> {
        user_state: user_state.clone(),
        user_dispatch: user_dispatch.clone(),
        error_dispatch: error_dispatch.clone(),
        set_all: set_all.clone(),
    };
    let delete_one = callbacks.clone().delete_one_cb(url.clone());
    let fetch_all = callbacks.clone().fetch_all_cb(url.clone());
    use_effect_with_deps(move |_| fetch_all.emit(Default::default()), ());
    html! {
        <>
            <ybc::Tile vertical=true size={Twelve}>
                <h1 class={"title algolia-lvl0"}>{props.title.clone()}</h1>
                <button class="is-small" aria-label="close" onclick={move |_| toggle_callback.emit(ToggleState::Show) }>{"Add [+]"}</button>
                <hr />
                if props.form.is_some() {
                  <CrudForm state={*toggle_form_cloned} close_cb={toggle_callback_cloned} form={props.form.clone().unwrap()}/>
                }
                <CrudTable<M> list={clone_all.to_vec()} delete_cb={delete_one}/>
            </ybc::Tile>
        </>
    }
}
