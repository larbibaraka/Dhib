FROM rust:1-bookworm as Builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

FROM ubuntu:24.04

WORKDIR /app

COPY --from=Builder /app/target/release/dhib /app/dhib

ENTRYPOINT ["/app/dhib"]
