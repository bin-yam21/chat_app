# ---- Builder stage ----
FROM rust:1.87 as builder
WORKDIR /app

# Install build dependencies for SQLx (needs libpq-dev)
RUN apt-get update && apt-get install -y libpq-dev

# Cache dependencies (layer caching for fast rebuild)
COPY Cargo.toml Cargo.lock ./
RUN mkdir src
RUN echo "fn main(){}" > src/main.rs
RUN cargo build --release || true

# Now copy the real source
COPY src ./src
COPY migrations ./migrations
COPY .env .env

RUN cargo build --release

# ---- Runtime stage ----
FROM debian:bookworm-slim
WORKDIR /app

# Install runtime dependencies and build tools for sqlx-cli
RUN apt-get update && apt-get install -y \
    libpq-dev \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
 && rm -rf /var/lib/apt/lists/*

# Install sqlx-cli binary (to run migrations)
RUN curl -sSL https://sh.rustup.rs | bash -s -- -y \
    && ~/.cargo/bin/cargo install sqlx-cli --no-default-features --features postgres \
    && ln -s ~/.cargo/bin/sqlx /usr/local/bin/sqlx

# Copy the compiled binary and other files
COPY --from=builder /app/target/release/chat_app ./chat_app
COPY migrations ./migrations
COPY .env .env

EXPOSE 3000

# ✅ Run migrations before starting the server
CMD ["sh", "-c", "sqlx migrate run && ./chat_app"]
