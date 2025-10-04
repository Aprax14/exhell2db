FROM rust:1.89-slim-bookworm AS builder
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    pkg-config \
    bash \
    wget \
    curl \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY webserver/Cargo.toml webserver/
COPY exhell-utils/Cargo.toml exhell-utils/

RUN mkdir webserver/src \
    exhell-utils/src && \
    echo "fn main() {}" > webserver/src/main.rs && \
    echo "" > exhell-utils/src/lib.rs && \
    cargo fetch && \
    rm -rf webserver && \
    rm -rf exhell-utils && \
    cargo build --release -p webserver || true

COPY . .

RUN cargo build --release -p webserver

FROM debian:bookworm-slim
WORKDIR /app

COPY --from=builder /app/target/release/webserver .
CMD ["./webserver"]
