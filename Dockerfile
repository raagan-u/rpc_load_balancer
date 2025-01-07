# Build stage using rust:alpine
FROM rust:1.83-alpine3.20 AS builder

WORKDIR /app

RUN apk add --no-cache \
    build-base \
    perl \
    pkgconfig \
    libffi-dev \
    musl-dev \
    musl \
    openssl \ 
    cmake 


RUN apk add --no-cache openssl-dev

RUN apk add --no-cache openssl-libs-static

COPY Cargo.toml Cargo.lock ./

COPY ./src ./src

RUN cargo build --release


FROM alpine:latest

WORKDIR /app

COPY --from=builder /app/target/release/rpc_load_balancer /usr/local/bin/rpc_load_balancer

EXPOSE 8080

CMD ["/usr/local/bin/rpc_load_balancer"]
