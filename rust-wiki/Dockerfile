FROM rust:latest as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY src ./src
COPY src/data ./src/data
COPY src/Article ./src/Article/

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/rust-wiki /app/app
COPY src/data ./src/data
COPY src/Article ./src/Article
COPY src ./src

CMD ["./app"]
