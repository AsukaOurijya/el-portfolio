FROM rust:1.88 AS builder

WORKDIR /app

RUN cargo install dioxus-cli --version 0.7.4 \
    && rustup target add wasm32-unknown-unknown

COPY Cargo.toml Cargo.lock ./
COPY Dioxus.toml ./
COPY rust-toolchain.toml ./
COPY src ./src
COPY assets ./assets
COPY tailwind.css ./tailwind.css

RUN dx build --platform web --release

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/dx/portfolio/release/web/server /app/portfolio
COPY --from=builder /app/target/dx/portfolio/release/web/public /app/public
COPY --from=builder /app/Dioxus.toml /app/Dioxus.toml

ENV IP=0.0.0.0

CMD ["/app/portfolio"]
