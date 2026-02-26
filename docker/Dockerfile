# Build stage: server only (no other workspace apps)
FROM rust:1-bookworm AS builder
WORKDIR /app

# Use minimal workspace that only has server + libs/core (no cli, etc.)
COPY docker/Cargo.toml.server ./Cargo.toml
COPY Cargo.lock ./
COPY libs ./libs
COPY apps/server ./apps/server

RUN cargo build -p featureflags-server --release

# Runtime stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app

ENV FLAGS_PATH=/data/flags.yml

COPY flags/flags.yml /data/flags.yml
COPY --from=builder /app/target/release/featureflags-server /usr/local/bin/featureflags-server

EXPOSE 8080

ENTRYPOINT ["/usr/local/bin/featureflags-server"]
