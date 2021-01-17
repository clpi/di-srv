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

## FROM alpine:latest
## COPY --from=builder /usr/src/div/target/release/divapi .
## COPY --from=builder /usr/local/cargo/bin/divapi .
## USER 1000
## CMD ["./divapi"]
##
#ARG BASE_IMAGE=ekidd/rust-musl-builder:latest

## Our first FROM statement declares the build environment.
#FROM ${BASE_IMAGE} AS builder

## Add our source code.
#ADD . ./

## Fix permissions on source code.
#RUN sudo chown -R rust:rust /home/rust

## Build our application.
#RUN cargo build --release

## Now, we need to build our _real_ Docker container, copying in `rust-actix-example`.
#FROM alpine:latest
#RUN apk --no-cache add ca-certificates
#COPY --from=builder \
#    /home/rust/src/target/x86_64-unknown-linux-musl/release/rust-actix-example \
#    /usr/local/bin/
#CMD /usr/local/bin/rust-actix-example
