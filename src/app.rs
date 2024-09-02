use dioxus::prelude::*;

mod home_page;
use home_page::HomePage;

#[component]
pub fn App() -> Element {
    rsx! {
        HomePage {}
    }
}
