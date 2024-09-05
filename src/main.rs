#![allow(non_snake_case)]

pub mod app;
pub mod pages;
mod components;

use app::App;

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}