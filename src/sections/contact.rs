use dioxus::prelude::*;

#[component]
pub fn Contact() -> Element {
    rsx! {
        div {
            class: "contact-card reveal-scroll reveal-scroll--card",

            form {
                class: "contact-form",

                div {
                    class: "contact-form__row",

                    label {
                        class: "contact-form__field",
                        span { class: "contact-form__label", "Name" }
                        input {
                            class: "contact-form__input",
                            r#type: "text"
                        }
                    }

                    label {
                        class: "contact-form__field",
                        span { class: "contact-form__label", "Email" }
                        input {
                            class: "contact-form__input",
                            r#type: "email"
                        }
                    }
                }

                label {
                    class: "contact-form__field",
                    span { class: "contact-form__label", "Subject" }
                    input {
                        class: "contact-form__input",
                        r#type: "text"
                    }
                }

                label {
                    class: "contact-form__field",
                    span { class: "contact-form__label", "Message" }
                    textarea {
                        class: "contact-form__textarea"
                    }
                }

                div {
                    class: "contact-form__actions",
                    button {
                        class: "contact-form__submit",
                        r#type: "submit",
                        "Submit"
                    }
                }
            }
        }
    }
}
