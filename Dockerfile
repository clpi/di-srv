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


#FROM alpine:latest AS builder
#RUN apk update --quiet
#RUN apk add curl
#RUN apk add build-base
#RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
#WORKDIR .
#COPY . .
#RUN $HOME/.cargo/bin/cargo build --release

## Run
#FROM alpine:latest
#RUN apk add -q --no-cache libgcc tini
#COPY --from=compiler /meilisearch/target/release/meilisearch .
#ENV PORT 0.0.0.0:7777
#EXPOSE  7777/tcp
#ENTRYPOINT ["tini", "--"]
#CMD     ./meilisearch
