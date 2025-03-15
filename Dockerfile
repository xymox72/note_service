FROM rust:1.85.0-alpine3.21 AS builder

RUN apk add --no-cache musl-dev openssl-dev pkgconfig openssl-libs-static

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo fetch


COPY . .
RUN cargo build --release --locked


FROM alpine:latest

RUN apk add --no-cache libssl3

WORKDIR /app

COPY --from=builder /app/target/release/note_service /usr/local/bin/note_service

ENV RUST_BACKTRACE=1
CMD ["note_service"]
