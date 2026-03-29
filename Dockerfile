# Stage 1: Build the application
FROM rust:latest AS builder

# Install nightly toolchain and wasm target
RUN rustup toolchain install nightly && \
    rustup default nightly && \
    rustup target add wasm32-unknown-unknown

# Install cargo-leptos
RUN cargo install cargo-leptos --version "^0.2" --locked 2>/dev/null || \
    cargo install cargo-leptos --locked

# Install Node.js for sass/css processing
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt-get install -y nodejs

WORKDIR /app

# Copy dependency files first for caching
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./
COPY vendor/ ./vendor/

# Create dummy src to build dependencies
RUN mkdir -p src && echo "fn main() {}" > src/main.rs && \
    echo "pub fn main() {}" > src/lib.rs && \
    cargo build --release 2>/dev/null || true

# Copy full source
COPY . .

# Build the full application using cargo-leptos
RUN cargo leptos build --release

# Stage 2: Runtime image
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the built binary
COPY --from=builder /app/target/release/todomvc ./todomvc

# Copy the site directory (CSS, JS, WASM)
COPY --from=builder /app/target/site ./target/site

# Copy migrations
COPY --from=builder /app/migrations ./migrations

# Copy env file
COPY --from=builder /app/.env ./.env

EXPOSE 8080

ENV RUST_LOG=info

CMD ["./todomvc"]
