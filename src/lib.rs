mod api;
mod components;
mod types;
mod pages;

use pages::Home;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Home>::new().mount_to_body();
}
