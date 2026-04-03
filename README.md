# Portfolio

This project is a Dioxus 0.7 portfolio app with fullstack contact form support.

## Prerequisites

1. Install Rust.

Windows:

```powershell
winget install Rustlang.Rustup
```

macOS / Linux:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verify the toolchain:

```bash
rustc --version
cargo --version
```

2. Install the Dioxus CLI:

```bash
cargo install dioxus-cli
```

3. Add the web target if you want to build the browser client manually:

```bash
rustup target add wasm32-unknown-unknown
```

This project is now fullstack, so `dx serve` is the normal way to run it. The wasm target is still useful for web builds, but it is not the main setup step anymore.

## Environment Setup

Create a `.env` file in the project root with:

```env
RESEND_API_KEY=re_xxxxxxxxx
CONTACT_FROM_EMAIL=onboarding@resend.dev
CONTACT_FROM_NAME="Portfolio Visitors"
CONTACT_TO_EMAIL=your@email.com
```

Notes:

- `CONTACT_FROM_EMAIL` must be allowed by Resend.
- For real delivery, verify your own domain in Resend and use a sender on that domain.
- `CONTACT_TO_EMAIL` is the inbox that receives contact submissions.

## Install Dependencies

From the project root:

```bash
cargo fetch
```

Current project setup in [Cargo.toml](/c:/Users/mazka/Documents/Rust/web_portfolio/portfolio/Cargo.toml):

- Dioxus `0.7.1`
- `router` and `fullstack` enabled
- custom features: `web`, `desktop`, `mobile`, `server`
- `default = []`, so feature selection matters

## Run In Development

Start the app from the project root:

```bash
dx serve
```

This runs the app in the normal Dioxus dev flow and serves the fullstack contact endpoint.

## Useful Commands

Run an explicit web/fullstack dev build:

```bash
dx serve --platform web
```

Create a release web build:

```bash
dx build --platform web --release
```

Fetch dependencies without building:

```bash
cargo fetch
```

## Contact Form

The contact form is implemented with a Dioxus server function and Resend email delivery.

If submissions fail:

- confirm `.env` exists in the project root
- restart the dev server after changing `.env`
- confirm the Resend sender is allowed
- confirm the recipient email is valid
