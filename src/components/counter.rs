#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn Counter() -> Element {
    let mut _count = use_signal(|| 0);
    rsx! {
        h1 { class: "text-white", "High-Five counter: {_count}" }
        button {
            class: "text-white",
            onclick: move |_| _count += 1,
            "Up high!"
        }
        button {
            class: "text-white",
            onclick: move |_| _count -= 1,
            "Down low!"
        }
    }
}
