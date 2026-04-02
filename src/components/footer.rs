use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            class: "site-footer reveal-scroll reveal-scroll--footer",
            p { "2026 All Rights Reserved. Made in Rust btw." }
        }
    }
}
