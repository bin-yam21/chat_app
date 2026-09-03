# ---- Builder stage ----
FROM rust:1.87 as builder
WORKDIR /app

# Install build dependencies for SQLx (needs libpq-dev)
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*

# Cache dependencies (layer caching for fast rebuilds): build a dummy binary
# with only the manifests so `cargo build` compiles all deps first.
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main(){}" > src/main.rs
RUN cargo build --release || true
RUN rm -rf src

# Now copy the real source + migrations and build for real. Migrations are
# embedded into the binary via sqlx::migrate!, so they must be present at
# build time. No .env is copied — configuration comes from the environment
# (DATABASE_URL, JWT_SECRET, PORT) provided by the host.
COPY src ./src
COPY migrations ./migrations
RUN cargo build --release

# ---- Runtime stage ----
FROM debian:bookworm-slim
WORKDIR /app

# Runtime-only dependencies (Postgres client libs + CA certs for TLS).
RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
 && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary and the migration files. The server applies
# migrations itself on startup, so no sqlx-cli is needed in the image.
COPY --from=builder /app/target/release/chat_app ./chat_app
COPY migrations ./migrations

# Documentation only; the app actually binds to $PORT (set by the host).
EXPOSE 3000

CMD ["./chat_app"]
