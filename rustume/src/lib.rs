// src/lib.rs
#![recursion_limit = "2048"]

// Modules

mod api;
mod app;
mod components;
mod pages;
mod routes;
mod types;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<app::App>::new().mount_to_body();
}
