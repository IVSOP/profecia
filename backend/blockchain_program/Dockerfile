FROM rust:slim-bookworm AS builder

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        curl \
        ca-certificates \
        libudev-dev \
        libssl-dev \
        pkg-config \
        build-essential \
        bzip2 \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Install Solana CLI (provides cargo-build-sbf)
RUN curl --proto '=https' --tlsv1.2 -sSfL https://solana-install.solana.workers.dev | bash
ENV PATH="/root/.local/share/solana/install/active_release/bin:${PATH}"

WORKDIR /workspace

# Copy blockchain_core dependency first (shared crate)
COPY blockchain_core/ blockchain_core/

# Copy blockchain_program source
COPY blockchain_program/ blockchain_program/

# Copy keypair for deployment
RUN mkdir -p blockchain_program/target/deploy \
    && cp blockchain_program/PRIVATE_KEY/ProirMXDTFF4AEqGyZVKPhWte4chANDd1c4Y8w7Nsd4.json \
       blockchain_program/target/deploy/blockchain_program-keypair.json

# Build the Solana program
WORKDIR /workspace/blockchain_program
RUN cargo-build-sbf

# --- Output stage: copy artifacts to volume mount points ---
FROM debian:bookworm-slim

# .so goes into /program/so (mounted as surfpool's /workspace/target/deploy)
COPY --from=builder /workspace/blockchain_program/target/deploy/blockchain_program.so /program/so/
COPY --from=builder /workspace/blockchain_program/target/deploy/blockchain_program-keypair.json /program/so/

# Keys go into /program/keys (mounted as surfpool's /root/.config/solana)
COPY --from=builder /workspace/blockchain_program/PRIVATE_KEY/ /program/keys/

# Container just starts, copies artifacts to volumes, and exits
CMD ["echo", "Program artifacts ready"]
