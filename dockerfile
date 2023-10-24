FROM rust:1.67-slim as builder
WORKDIR /app
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release
EXPOSE 3000
CMD ["./target/release/rust_mvc_web"]