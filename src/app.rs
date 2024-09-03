#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::page::HomePage;

#[component]
pub fn App() -> Element {
    let mut _count = use_signal(|| 0);

    rsx! {
        HomePage {}
        h1 { class: "text-white", "High-Five counter: {_count}" }
        button { class: "text-white", onclick: move |_| _count += 1, "Up high!" }
        button { class: "text-white", onclick: move |_| _count -= 1, "Down low!" }
    }
}
