#![allow(non_snake_case)]

use dioxus::prelude::*;

#[component]
pub fn HomePage() -> Element {
    const STYLE: &str = manganis::mg!(file("src/styles/main.css"));
    const SVG_FILE: &str = manganis::mg!(file("src/assets/header.svg"));

    rsx! {
        img { src: SVG_FILE, id: "header" }
        link { rel: "stylesheet", href: STYLE }
        div { id: "links",
            a { target: "_blank", href: "https://dioxuslabs.com/learn/0.5/", "📚 Learn Dioxus" }
            a { target: "_blank", href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
            a { target: "_blank", href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
            a { target: "_blank", href: "https://github.com/DioxusLabs/dioxus-std", "⚙️ Dioxus Standard Library" }
            a { target: "_blank", href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
            a { target: "_blank", href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
        }
    }
}