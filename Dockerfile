# Build Stage
FROM rust:1.47-slim-buster AS builder
WORKDIR /usr/src/div
RUN rustup target add x86_64-unknown-linux-musl

COPY . .
RUN apt-get update -y && apt-get upgrade -y && apt-get install clang llvm-dev libclang-dev pkg-config libssl-dev -y
#COPY . . 
RUN cargo build --release
RUN cargo install --target x86_64-unknown-linux-musl --path .


# Bundle Stage
FROM scratch
COPY --from=builder /usr/local/cargo/bin/divapi .
USER 1000
CMD ["./divapi"]
