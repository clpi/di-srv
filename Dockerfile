# Build Stage
FROM rust:1.47-slim AS builder
WORKDIR /usr/src/div
RUN sudo apt install libssl-dev -y
COPY . .
#RUN cargo build --release
#RUN cargo install --target x86_64-unknown-linux-musl --path .
RUN cargo install --path .


# Bundle Stage
FROM alpine:latest
RUN apk add -q --no-cache libgcc
COPY --from=builder /usr/src/div/target/release/divapi .
COPY --from=builder /usr/local/cargo/bin/divapi .
USER 1000
CMD ["./divapi"]
