use crate::components::card::Card;
use dioxus::prelude::*;

#[component]
pub fn Skills() -> Element {
    rsx! {
        Card {
            div {
                class: "skills-card",

                div {
                    class: "skills-card__group",

                    p {
                        class: "terminal-command",
                        "azka@portfolio:~$ ls technical_skills"
                    }

                    div {
                        class: "skills-card__grid skills-card__grid--technical",

                        p { "python" }
                        p { "springboot" }
                        p { "html" }
                        p { "docker" }
                        p { "postgresql" }
                        p { "java" }
                        p { "rust" }
                        p { "tailwind css" }
                        p { "github" }
                        p { "" }
                        p { "django" }
                        p { "dioxus" }
                        p { "react.js" }
                        p { "linux" }
                        p { "" }
                    }
                }

                div {
                    class: "skills-card__group skills-card__group--soft",

                    p {
                        class: "terminal-command",
                        "azka@portfolio:~$ ls soft_skills"
                    }

                    div {
                        class: "skills-card__grid skills-card__grid--soft",

                        p { "problem solving" }
                        p { "critical thinking" }
                        p { "communication" }
                        p { "emotional intelligence" }
                        p { "teamwork" }
                        p { "time management" }
                    }
                }
            }
        }
    }
}
