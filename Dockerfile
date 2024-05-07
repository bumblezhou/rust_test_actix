FROM rust:latest as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM ubuntu:22.04

EXPOSE 8080

WORKDIR /app

COPY --from=builder /app ./

CMD ["./target/release/rust_test_actix"]
