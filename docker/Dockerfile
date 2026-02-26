# Build stage: server only (musl for static binary, small image)
FROM rust:1-alpine AS builder
RUN apk add --no-cache musl-dev
WORKDIR /app

COPY docker/Cargo.toml.server ./Cargo.toml
COPY Cargo.lock ./
COPY libs ./libs
COPY apps/server ./apps/server

RUN cargo build -p featureflags-server --release && \
    strip /app/target/release/featureflags-server 2>/dev/null || true

# Runtime stage (< 50MB)
FROM alpine:3.19
RUN apk add --no-cache ca-certificates
WORKDIR /app
ENV FLAGS_PATH=/data/flags.yml

COPY --from=builder /app/target/release/featureflags-server /usr/local/bin/featureflags-server

EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/featureflags-server"]
