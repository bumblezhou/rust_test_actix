FROM rust:latest as builder

WORKDIR /app

COPY . .

# Set proxy environment variables
ENV http_proxy=http://your_proxy_server:proxy_port
ENV https_proxy=http://your_proxy_server:proxy_port

RUN cargo build --release

FROM ubuntu:22.04

EXPOSE 8080

WORKDIR /app

COPY --from=builder /app ./

CMD ["./target/release/rust_test_actix"]
