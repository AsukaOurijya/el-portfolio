use dioxus::prelude::*;

const TODO_IMG: Asset = asset!("/assets/img/todolist.jpeg");
const SNAKE_IMG: Asset = asset!("/assets/img/snakegame.jpg");
const CALCULATOR_IMG: Asset = asset!("/assets/img/mycalculatorgw.png");

#[component]
pub fn Works() -> Element {
    rsx! {
        div {
            class: "works-grid",

            ProjectCard {
                image: TODO_IMG,
                title: "Get Things Done!".to_string(),
                github_href: "https://github.com/AsukaOurijya/get-things-done".to_string(),
                live_href: "https://get-things-done-iota.vercel.app/".to_string(),
            }
            ProjectCard {
                image: SNAKE_IMG,
                title: "snakey breekey".to_string(),
                github_href: "https://github.com/AsukaOurijya/Snakey-Breekey".to_string(),
                live_href: "".to_string(),
            }
            ProjectCard {
                image: CALCULATOR_IMG,
                title: "My Calculator Gw".to_string(),
                github_href: "https://github.com/AsukaOurijya/my-calculator-gw".to_string(),
                live_href: "https://asukaourijya.github.io/my-calculator-gw/".to_string(),
            }
        }
    }
}

#[component]
fn ProjectCard(image: Asset, title: String, github_href: String, live_href: String) -> Element {
    let has_live = !live_href.is_empty();

    rsx! {
        article {
            class: "project-card reveal-scroll reveal-scroll--card",

            img {
                class: "project-card__image",
                src: image,
                alt: title.clone()
            }

            h3 {
                class: "project-card__title",
                "{title}"
            }

            div {
                class: "project-card__links",

                a {
                    href: github_href,
                    class: "project-card__link",
                    "GitHub"
                }

                if has_live {
                    a {
                        href: live_href,
                        class: "project-card__link",
                        "Link"
                    }
                }
            }
        }
    }
}
