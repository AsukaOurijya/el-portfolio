use crate::components::card::Card;
use dioxus::prelude::*;

const ROY_IMG: Asset = asset!("/assets/img/roy.jpg");

#[component]
pub fn About() -> Element {
    rsx! {
        Card {
            div {
                class: "about-card",

                p {
                    class: "terminal-command",
                    "azka@portfolio:~$ azkafetch"
                }

                div {
                    class: "about-card__layout",

                    div {
                        class: "about-card__sidebar",

                        img {
                            src: ROY_IMG,
                            class: "about-card__avatar"
                        }

                        div {
                            class: "about-card__actions",

                            button {
                                class: "about-card__button",
                                "Get My CV"
                            }

                            div {
                                class: "about-card__socials",

                                a {
                                    href: "#",
                                    class: "about-card__social about-card__social--github",
                                    aria_label: "GitHub",
                                    svg {
                                        class: "about-card__social-icon",
                                        view_box: "0 0 24 24",
                                        fill: "currentColor",
                                        path {
                                            d: "M12 0C5.37 0 0 5.37 0 12c0 5.3 3.44 9.8 8.2 11.39.6.11.82-.26.82-.58 0-.28-.01-1.04-.02-2.04-3.34.73-4.04-1.61-4.04-1.61-.55-1.38-1.33-1.75-1.33-1.75-1.09-.74.08-.72.08-.72 1.2.08 1.84 1.23 1.84 1.23 1.07 1.83 2.81 1.3 3.49 1 .11-.78.42-1.3.76-1.6-2.66-.31-5.47-1.33-5.47-5.9 0-1.3.47-2.37 1.23-3.2-.12-.3-.53-1.55.12-3.22 0 0 1.01-.32 3.3 1.22a11.42 11.42 0 0 1 6 0c2.29-1.54 3.29-1.22 3.29-1.22.66 1.67.25 2.92.13 3.22.77.83 1.23 1.9 1.23 3.2 0 4.59-2.81 5.59-5.49 5.89.43.37.81 1.1.81 2.22 0 1.6-.01 2.89-.01 3.28 0 .32.22.7.82.58A12.01 12.01 0 0 0 24 12c0-6.63-5.37-12-12-12Z"
                                        }
                                    }
                                }
                                a {
                                    href: "#",
                                    class: "about-card__social about-card__social--instagram",
                                    aria_label: "Instagram",
                                    svg {
                                        class: "about-card__social-icon",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "1.9",
                                        rect {
                                            x: "4.2",
                                            y: "4.2",
                                            width: "15.6",
                                            height: "15.6",
                                            rx: "4.4"
                                        }
                                        circle {
                                            cx: "12",
                                            cy: "12",
                                            r: "3.8"
                                        }
                                        circle {
                                            cx: "17.4",
                                            cy: "6.7",
                                            r: "0.9",
                                            fill: "currentColor",
                                            stroke: "none"
                                        }
                                    }
                                }
                                a {
                                    href: "#",
                                    class: "about-card__social about-card__social--linkedin",
                                    aria_label: "LinkedIn",
                                    svg {
                                        class: "about-card__social-icon",
                                        view_box: "0 0 24 24",
                                        fill: "currentColor",
                                        circle {
                                            cx: "6",
                                            cy: "8",
                                            r: "1.8"
                                        }
                                        path {
                                            d: "M4.5 10.2h3V19h-3zM10 10.2h2.88v1.2h.04c.4-.76 1.39-1.56 2.86-1.56 3.06 0 3.62 2.01 3.62 4.63V19h-3v-3.98c0-.95-.02-2.18-1.33-2.18-1.33 0-1.53 1.04-1.53 2.11V19h-3z"
                                        }
                                    }
                                }
                            }
                        }
                    }

                    div {
                        class: "about-card__content",

                        p { class: "about-card__name", "azka@portfolio" }
                        p { class: "about-card__divider", "--------------------" }

                        p { class: "about-card__line", "Fullname: Muhammad Azka Awliya" }
                        p { class: "about-card__line", "Education: CS @ Universitas Indonesia" }
                        p {
                            class: "about-card__line about-card__line--multiline",
                            "Interest: Software Engineering, Web Development,"
                            br {}
                            "Cloud Engineering, & DevOps"
                        }
                        p { class: "about-card__line", "Strength: problem solving, creativity, commitment" }
                        p {
                            class: "about-card__quote",
                            "Life Principle: “Abandoning your uniqueness is equivalent"
                            br {}
                            "to dying.” - Ryo Yamada, BTR"
                        }
                    }
                }
            }
        }
    }
}
