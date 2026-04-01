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
RUN cargo build --release
RUN cargo run --bin assets

# ---------- Runtime stage ----------
FROM gcr.io/distroless/cc-debian12

WORKDIR /app

COPY --from=builder /app/target/release/personal-website .

COPY templates ./templates
COPY static ./static
COPY readme.md ./readme.md

USER nonroot:nonroot

EXPOSE 3000

CMD ["./personal-website"]