# ---------- Planner stage ----------
FROM rust:latest AS planner
WORKDIR /app

RUN cargo install cargo-chef

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ---------- Builder stage ----------
FROM rust:latest AS builder
WORKDIR /app

ENV SQLX_OFFLINE=true

RUN cargo install cargo-chef

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo run --release --bin assets
RUN cargo build --release

# ---------- Runtime stage ----------
FROM debian:bookworm-slim
# For later:
#FROM gcr.io/distroless/cc-debian12

WORKDIR /app

COPY --from=builder /app/target/release/buildhaven .
COPY --from=builder /app/static ./static

COPY templates ./templates
COPY media ./media
COPY dist ./dist
COPY docs ./docs

RUN useradd -m nonroot
USER root
#USER nonroot:nonroot

EXPOSE 3000

CMD ["./buildhaven"]