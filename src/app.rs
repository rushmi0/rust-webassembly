#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::pages::HomePage;

#[component]
pub fn App() -> Element {
    const ICON: &str = manganis::mg!(file("src/assets/favicon.ico"));
    rsx! {
        link { rel: "icon", href: ICON }
        HomePage {}
    }
}
