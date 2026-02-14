#!/bin/bash
set -e

# Set up Solana PATH
export PATH="/root/.local/share/solana/bin:${PATH}"

# Copy private key to target/deploy
mkdir -p /workspace/blockchain_program/target/deploy
cp /workspace/blockchain_program/PRIVATE_KEY/*.json /workspace/target/deploy/blockchain_program-keypair.json

cd /workspace/blockchain_program

# sleep 1000000000000000000000000000000

# Build the Solana program
echo "Building Solana program..."
# TODO: PATH IS PROB WRONG? MAYBE MISSING SOURCING BASHRC?
# cargo build-sbf
# /root/.local/share/solana/install/releases/2.1.21/solana-release/bin/cargo-build-sbf
/root/.local/share/solana/install/active_release/bin/cargo-build-sbf

# # Run surfpool
# echo "Running surfpool..."
# # surfpool start
# # TODO: SAME
# /root/.local/bin/surfpool start
