# Build Stage
FROM rust:1.47-slim AS builder
WORKDIR /usr/src/div
RUN apt-get update && apt-get install -y pkg-config libssl-dev ca-certificates
COPY . .
RUN cargo build --target x86_64-unknown-linux-musl --release
#RUN cargo install --target x86_64-unknown-linux-musl --path .
RUN cargo install --path .


# Bundle Stage
FROM alpine:latest
COPY --from=builder /usr/src/div/target/release/divapi .
COPY --from=builder /usr/local/cargo/bin/divapi .
USER 1000
CMD ["./divapi"]
