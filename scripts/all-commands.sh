#!/bin/bash
set -e

# To format code
cargo fmt

# For linting
cargo clippy --all-targets --all-features -- -D warnings

# To run tests
cargo test

# To check SQLx query metadata
cargo sqlx prepare --check

# To print the project tree without target nor node_modules
tree -I 'target|node_modules'

# To update repository
git fetch origin
git reset --hard origin/main

# To pull latest images
podman-compose pull

# To Restart services
podman-compose up -d

# To clean dangling images
podman image prune -f

# Security audit
cargo audit

# Binary size analysis
cargo bloat --release --crates

# To build documentation
cargo doc --no-deps

# To benchmark
cargo bench || true

# In prod
sudo podman run -d \
  --name buildhaven-postgres \
  --network buildhaven-net \
  --env-file /etc/BuildHaven/.env \
  -v buildhaven-postgres-data:/var/lib/postgresql/data \
  docker.io/library/postgres:latest

# Build to (then ship to) prod
podman build --platform linux/arm64 \
  -t ghcr.io/vitor-curado/buildhaven:arm64 \
  .