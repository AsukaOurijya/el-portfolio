use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
struct ContactRequest {
    name: String,
    email: String,
    subject: String,
    message: String,
    website: String,
}

#[derive(Clone, Debug, PartialEq)]
enum ContactFeedback {
    Idle,
    Success(String),
    Error(String),
}

#[component]
pub fn Contact() -> Element {
    let mut name = use_signal(String::new);
    let mut email = use_signal(String::new);
    let mut subject = use_signal(String::new);
    let mut message = use_signal(String::new);
    let mut website = use_signal(String::new);
    let mut is_submitting = use_signal(|| false);
    let mut feedback = use_signal(|| ContactFeedback::Idle);

    rsx! {
        div {
            class: "contact-card reveal-scroll reveal-scroll--card",

            form {
                class: "contact-form",
                onsubmit: move |evt: FormEvent| async move {
                    evt.prevent_default();

                    if is_submitting() {
                        return;
                    }

                    let payload = ContactRequest {
                        name: name(),
                        email: email(),
                        subject: subject(),
                        message: message(),
                        website: website(),
                    };

                    is_submitting.set(true);
                    feedback.set(ContactFeedback::Idle);

                    match submit_contact(payload).await {
                        Ok(success_message) => {
                            name.set(String::new());
                            email.set(String::new());
                            subject.set(String::new());
                            message.set(String::new());
                            website.set(String::new());
                            feedback.set(ContactFeedback::Success(success_message));
                        }
                        Err(err) => {
                            feedback.set(ContactFeedback::Error(contact_error_message(err)));
                        }
                    }

                    is_submitting.set(false);
                },

                div {
                    class: "contact-form__row",

                    label {
                        class: "contact-form__field",
                        span { class: "contact-form__label", "Name" }
                        input {
                            class: "contact-form__input",
                            name: "name",
                            value: name(),
                            required: true,
                            autocomplete: "name",
                            r#type: "text",
                            oninput: move |evt| name.set(evt.value())
                        }
                    }

                    label {
                        class: "contact-form__field",
                        span { class: "contact-form__label", "Email" }
                        input {
                            class: "contact-form__input",
                            name: "email",
                            value: email(),
                            required: true,
                            autocomplete: "email",
                            r#type: "email",
                            oninput: move |evt| email.set(evt.value())
                        }
                    }
                }

                label {
                    class: "contact-form__field",
                    span { class: "contact-form__label", "Subject" }
                    input {
                        class: "contact-form__input",
                        name: "subject",
                        value: subject(),
                        required: true,
                        r#type: "text",
                        oninput: move |evt| subject.set(evt.value())
                    }
                }

                label {
                    class: "contact-form__field",
                    span { class: "contact-form__label", "Message" }
                    textarea {
                        class: "contact-form__textarea",
                        name: "message",
                        value: message(),
                        required: true,
                        oninput: move |evt| message.set(evt.value())
                    }
                }

                label {
                    class: "contact-form__honeypot",
                    aria_hidden: "true",
                    span { "Website" }
                    input {
                        name: "website",
                        tabindex: "-1",
                        autocomplete: "off",
                        value: website(),
                        oninput: move |evt| website.set(evt.value())
                    }
                }

                {match feedback() {
                    ContactFeedback::Idle => rsx! {},
                    ContactFeedback::Success(message) => rsx! {
                        p {
                            class: "contact-form__status contact-form__status--success",
                            "{message}"
                        }
                    },
                    ContactFeedback::Error(message) => rsx! {
                        p {
                            class: "contact-form__status contact-form__status--error",
                            "{message}"
                        }
                    },
                }}

                div {
                    class: "contact-form__actions",
                    button {
                        class: "contact-form__submit",
                        r#type: "submit",
                        disabled: is_submitting(),
                        if is_submitting() {
                            "Sending..."
                        } else {
                            "Submit"
                        }
                    }
                }
            }
        }
    }
}

#[post("/api/contact")]
async fn submit_contact(payload: ContactRequest) -> std::result::Result<String, ServerFnError> {
    if !payload.website.trim().is_empty() {
        return Ok("Message sent. I'll get back to you soon.".to_string());
    }

    validate_contact(&payload)?;

    #[cfg(feature = "server")]
    {
        send_contact_email(&payload).await?;
        Ok("Message sent. I'll get back to you soon.".to_string())
    }

    #[cfg(not(feature = "server"))]
    {
        let _ = payload;
        Err(server_error(
            503,
            "Contact submissions are unavailable in this build.",
        ))
    }
}

fn validate_contact(payload: &ContactRequest) -> std::result::Result<(), ServerFnError> {
    if payload.name.trim().is_empty()
        || payload.email.trim().is_empty()
        || payload.subject.trim().is_empty()
        || payload.message.trim().is_empty()
    {
        return Err(server_error(400, "Please fill in every field."));
    }

    if !payload.email.contains('@') {
        return Err(server_error(400, "Please enter a valid email address."));
    }

    Ok(())
}

fn server_error(code: u16, message: impl Into<String>) -> ServerFnError {
    ServerFnError::ServerError {
        message: message.into(),
        code,
        details: None,
    }
}

fn contact_error_message(error: ServerFnError) -> String {
    match error {
        ServerFnError::ServerError { message, .. } => message,
        other => other.to_string(),
    }
}

#[cfg(feature = "server")]
async fn send_contact_email(payload: &ContactRequest) -> std::result::Result<(), ServerFnError> {
    use resend_rs::types::CreateEmailBaseOptions;
    use resend_rs::Resend;

    let resend_api_key = env_required("RESEND_API_KEY")?;
    let to_email = env_required("CONTACT_TO_EMAIL")?;
    let from_email = env_required("CONTACT_FROM_EMAIL")?;
    let from_name = env_optional("CONTACT_FROM_NAME")
        .unwrap_or_else(|| "Portfolio Contact".to_string());

    let resend = Resend::new(&resend_api_key);
    let from = format!("{from_name} <{from_email}>");
    let to = [to_email];
    let subject = format!("Portfolio contact: {}", payload.subject.trim());
    let body = format!(
            "Name: {}\nEmail: {}\nSubject: {}\n\n{}",
            payload.name.trim(),
            payload.email.trim(),
            payload.subject.trim(),
            payload.message.trim()
        );
    let email = CreateEmailBaseOptions::new(from, to, subject).with_text(&body);

    let send_result = resend
        .emails
        .send(email)
        .await
        .map_err(|err| server_error(500, format!("Failed to send email with Resend: {err}")))?;

    let _message_id = send_result.id;

    Ok(())
}

#[cfg(feature = "server")]
fn env_required(name: &str) -> std::result::Result<String, ServerFnError> {
    load_server_env();

    std::env::var(name)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| server_error(500, format!("Missing required environment variable: {name}")))
}

#[cfg(feature = "server")]
fn env_optional(name: &str) -> Option<String> {
    load_server_env();

    std::env::var(name)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

#[cfg(feature = "server")]
fn load_server_env() {
    let env_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(".env");
    let _ = dotenvy::from_path_override(&env_path);
}
