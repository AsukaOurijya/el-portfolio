use dioxus::prelude::*;

mod components;
mod sections;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link {
            rel: "preconnect",
            href: "https://fonts.googleapis.com"
        }
        document::Link {
            rel: "preconnect",
            href: "https://fonts.gstatic.com",
            crossorigin: "anonymous"
        }
        document::Stylesheet {
            href: "https://fonts.googleapis.com/css2?family=Alata&family=Lobster+Two:wght@700&display=swap"
        }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } 
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        
        Router::<Route> {}
    }
}


/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        div {
            div {
                class: "portfolio-page",
                section {
                    id: "about",
                    class: "portfolio-section portfolio-section--about",
                    crate::sections::about::About {}
                }

                h2 {
                    id: "skills",
                    class: "section-heading reveal-scroll reveal-scroll--heading",
                    "skills"
                }

                section {
                    class: "portfolio-section portfolio-section--skills",
                    sections::skills::Skills {}
                }

                h2 {
                    id: "works",
                    class: "section-heading reveal-scroll reveal-scroll--heading",
                    "works"
                }

                section {
                    class: "portfolio-section portfolio-section--works",
                    sections::works::Works {}
                }

                h2 {
                    id: "contact",
                    class: "section-heading reveal-scroll reveal-scroll--heading",
                    "contact me"
                }

                section {
                    class: "portfolio-section portfolio-section--contact",
                    sections::contact::Contact {}
                }
            }

            components::footer::Footer {}
        }
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div {
            nav {
                class: "site-nav",
                div {
                    class: "site-nav__inner",
                    a {
                        href: "#about",
                        class: "site-nav__brand",
                        "AKZ.dev"
                    }

                    div {
                        class: "site-nav__links",
                        a { href: "#about", "about" }
                        a { href: "#skills", "skills" }
                        a { href: "#works", "works" }
                        a { href: "#contact", "contact" }
                    }
                }
            }
            Outlet::<Route> {}
        }
    }
}
