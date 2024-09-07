#![allow(non_snake_case)]

use dioxus::prelude::*;

use crate::components::Banner;

#[component]
pub fn HomePage() -> Element {
    rsx! {
        Banner {}
    }
}