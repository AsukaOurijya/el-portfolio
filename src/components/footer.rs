use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer {
            class: "site-footer",
            p { "2026 All Rights Reserved. Made in Rust btw." }
        }
    }
}
