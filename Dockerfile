FROM rust:1.45-slim

WORKDIR /usr/src/div/server

COPY . .

RUN apt-get update -y && apt-get upgrade -y && apt-get install clang llvm-dev libclang-dev pkg-config libssl-dev -y

RUN cargo build --release

RUN cargo install --path ./

ENV HOST_PORT 80
ENV ENVIRONMENT DEV

EXPOSE 80

CMD ["/usr/local/cargo/bin/div"]
