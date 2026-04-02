# Portfolio

This project is a Dioxus 0.7 web app.

## Install the framework

1. Install Rust:

Windows:

```powershell
winget install Rustlang.Rustup
```

macOS:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Linux:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

After installation, verify Rust is available:

```bash
rustc --version
cargo --version
```

2. Install the Dioxus CLI on any platform:

```bash
cargo install dioxus-cli
```

3. Add the web target used by this project:

```bash
rustup target add wasm32-unknown-unknown
```

4. Verify the CLI is installed:

```bash
dx --version
```

## Install project dependencies

From the project root, fetch the Rust dependencies:

```bash
cargo fetch
```

This project already declares Dioxus in [`Cargo.toml`](/c:/Users/mazka/Documents/Rust/web_portfolio/portfolio/Cargo.toml) with the `router` feature and enables the `web` feature by default.

## Run the development server

Start the Dioxus web server from the project root:

```bash
dx serve
```

The CLI will build the app, start a local dev server, and watch for file changes.

## Optional commands

Run the app explicitly for the web platform:

```bash
dx serve --platform web
```

Create an optimized production build:

```bash
dx build --platform web --release
```
