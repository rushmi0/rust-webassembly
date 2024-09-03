#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::page::home_page::HomePage;

#[component]
pub fn App() -> Element {
    rsx! {
        HomePage {}
    }
}
