# Build Stage
FROM rust:1.47-slim AS builder
WORKDIR /usr/src/div
RUN apt-get update && apt-get install -y pkg-config libssl-dev ca-certificates
COPY . .
# RUN cargo build --target x86_64-unknown-linux-musl --release
RUN cargo install --path .

FROM scratch
COPY --from=builder /usr/local/cargo/bin/divapi .
CMD ["/divapi"]

# FROM alpine:latest
# COPY --from=builder /usr/src/div/target/release/divapi .
# COPY --from=builder /usr/local/cargo/bin/divapi .
# USER 1000
# CMD ["./divapi"]
