# ---------- Build stage ----------
FROM rust:latest AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

# ---------- Runtime stage ----------
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/personal-website /app/personal-website

COPY templates ./templates
COPY static ./static
COPY readme.md ./readme.md

EXPOSE 3000

CMD ["./personal-website"]

# podman build --no-cache -t personal-website .
# podman run -it --rm -p 3000:3000 personal-website