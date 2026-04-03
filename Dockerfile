FROM rust:1.88 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY Dioxus.toml ./
COPY rust-toolchain.toml ./
COPY src ./src
COPY assets ./assets
COPY tailwind.css ./tailwind.css

RUN cargo build --release --features server,web

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/portfolio /app/portfolio
COPY --from=builder /app/assets /app/public/assets
COPY --from=builder /app/Dioxus.toml /app/Dioxus.toml

ENV IP=0.0.0.0

CMD ["/app/portfolio"]
