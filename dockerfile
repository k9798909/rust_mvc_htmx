FROM rust:1.67-slim as builder
WORKDIR /usr/src/rust_app
COPY . .

ENV SQLX_OFFLINE true
RUN cargo install --path .

CMD ["rust_app"]