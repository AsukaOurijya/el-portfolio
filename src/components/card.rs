use dioxus::prelude::*;

#[component]
pub fn Card(children: Element) -> Element {
    rsx! {
        div {
            class: "terminal-card",

            div {
                class: "terminal-card__topbar",

                span {
                    class: "terminal-card__plus",
                    "+"
                }

                span {
                    class: "terminal-card__title",
                    "azka@portfolio:~"
                }

                div {
                    class: "terminal-card__actions",
                    span { class: "terminal-card__action", "-" }
                    span { class: "terminal-card__action terminal-card__action--square", "□" }
                    span { class: "terminal-card__action", "x" }
                }
            }

            div {
                class: "terminal-card__body",
                {children}
            }
        }
    }
}
