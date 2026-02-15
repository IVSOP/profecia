FROM rust:1-bookworm AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy workspace manifests first for better layer caching
COPY Cargo.toml Cargo.lock* ./
COPY api/Cargo.toml api/Cargo.toml
COPY blockchain_client/Cargo.toml blockchain_client/Cargo.toml
COPY blockchain_core/Cargo.toml blockchain_core/Cargo.toml

# Create dummy sources so cargo can resolve the workspace
RUN mkdir -p api/src && echo "fn main() {}" > api/src/main.rs \
    && mkdir -p blockchain_client/src && echo "pub fn dummy() {}" > blockchain_client/src/lib.rs \
    && echo "fn main() {}" > blockchain_client/src/main.rs \
    && mkdir -p blockchain_core/src && echo "pub fn dummy() {}" > blockchain_core/src/lib.rs \
    && mkdir -p blockchain_core/src/accounts && touch blockchain_core/src/accounts/mod.rs

# Pre-build dependencies
RUN cargo build --release -p api 2>/dev/null || true

# Copy actual sources
COPY api/ api/
COPY blockchain_client/ blockchain_client/
COPY blockchain_core/ blockchain_core/

# Touch source files to invalidate the dummy builds
RUN find api/src blockchain_client/src blockchain_core/src -name "*.rs" -exec touch {} +

RUN cargo build --release -p api

# --- Production stage ---
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/api ./api

ENV RUST_LOG=info

EXPOSE 3000

CMD ["./api"]
