FROM rust:latest

WORKDIR /app

COPY Cargo.toml /app
COPY Cargo.lock /app
COPY src /app/src
ENV RUST_BACKTRACE=1

RUN cargo build --release

EXPOSE 8089

ENTRYPOINT ["/app/target/release/proxy"]
