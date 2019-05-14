FROM shepmaster/rust-nightly:latest AS builder
RUN apt-get update && apt-get -y install libpq-dev

WORKDIR /app
COPY Cargo.toml Cargo.lock diesel.toml /app/
COPY src /app/src
ENV RUST_BACKTRACE=1
RUN cargo build --release --locked

EXPOSE 8089
ENTRYPOINT ["/app/target/release/rocket_proxy"]
