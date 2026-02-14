FROM rust:slim-bookworm

# # Install system dependencies
# RUN apt-get update && apt-get install -y \
#     curl \
#     ca-certificates \
#     build-essential \
#     pkg-config \
#     libssl-dev \
#     libudev-dev \
#     bzip2 \
#     && rm -rf /var/lib/apt/lists/*

# # Install Solana tools from GitHub release
# RUN SOLANA_VERSION="v1.18.26" && \
#     curl -L "https://github.com/solana-labs/solana/releases/download/${SOLANA_VERSION}/solana-release-x86_64-unknown-linux-gnu.tar.bz2" -o /tmp/solana.tar.bz2 && \
#     tar -xjf /tmp/solana.tar.bz2 -C /tmp && \
#     mv /tmp/solana-release /root/.local/share/solana && \
#     rm /tmp/solana.tar.bz2
# ENV PATH="/root/.local/share/solana/bin:${PATH}"

# # Verify Solana installation
# RUN solana --version && cargo --version

# # Install cargo-build-sbf and surfpool
# RUN cargo install cargo-build-sbf || echo "cargo-build-sbf installation attempted" && \
#     cargo install surfpool || echo "surfpool installation attempted"

# WORKDIR /workspace

# # Copy and setup entrypoint script
# COPY entrypoint.sh /entrypoint.sh
# RUN chmod +x /entrypoint.sh

# ENTRYPOINT ["/entrypoint.sh"]

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        curl \
        ca-certificates \
        libudev-dev \
        libssl-dev \
        pkg-config \
        build-essential \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

    
# Install Solana
# ARG SOLANA_VERSION=v2.1.21
# RUN sh -c "$(curl -sSfL https://release.solana.com/v${SOLANA_VERSION}/install)"
# ENV PATH="${HOME}/.local/share/solana/install/active_release/bin:${PATH}"
RUN curl --proto '=https' --tlsv1.2 -sSfL https://solana-install.solana.workers.dev | bash

# # install surfpool
# RUN curl -sL "https://run.surfpool.run/" | bash

WORKDIR /workspace

COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
