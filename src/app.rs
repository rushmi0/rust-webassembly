use dioxus::prelude::*;
use crate::page::home_page::HomePage;

#[component]
pub fn App() -> Element {
    rsx! {
        link { rel: "stylesheet", href: "./main.css" }
        HomePage {}
    }
}
