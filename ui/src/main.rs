mod components;
mod stores;
mod api;
mod routes;
mod contexts;

use crate::contexts::{UserContext, ErrorContext};
use crate::routes::{Route};
use crate::components::app::App;


fn main() {
    yew::Renderer::<App>::new().render();
}