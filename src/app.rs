#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::pages::HomePage;

const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));
const STYLE: &str = manganis::mg!(file("src/main.css"));
const ICON: &str = manganis::mg!(file("src/assets/favicon.ico"));


#[component]
pub fn App() -> Element {
    rsx! {
        link { rel: "icon", href: ICON }
        link { rel: "stylesheet", href: STYLE }
        HomePage {}
    }
}
