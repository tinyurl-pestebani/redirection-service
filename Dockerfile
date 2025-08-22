FROM rust:1.88.0-alpine3.22 AS builder
LABEL authors="pablo"

WORKDIR /usr/src/app

COPY ./src ./src
COPY ./Cargo.toml .
COPY ./Cargo.lock .

WORKDIR /usr/src/app/redirection-service

RUN apk add --no-cache musl-dev pkgconf build-base openssl-dev protobuf-dev perl && export RUST_BACKTRACE=full && export OPENSSL_LIB_DIR=/usr && export PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig && export RUSTFLAGS='-C target-feature=+crt-static' && rustup target add x86_64-unknown-linux-musl && cargo build --target x86_64-unknown-linux-musl --release


FROM scratch

COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/redirection-service /redirection-service

ENTRYPOINT ["/redirection-service"]
