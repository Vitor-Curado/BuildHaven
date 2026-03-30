# 🔄 CI / Tooling

Continuous integration and development tooling ensure consistent builds and code quality.

## CI Pipeline

- **System:** Woodpecker CI

## Rust Tooling

- Formatting: `cargo fmt`
- Linting: `cargo clippy --pedantic`
- Testing: `cargo test`

## Container Builds

- Multi-stage Dockerfile with `cargo chef` for caching