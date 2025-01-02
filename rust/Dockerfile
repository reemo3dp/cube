FROM rust AS builder
WORKDIR /app

COPY Cargo.lock Cargo.toml ./
COPY src src
RUN cargo build --release

FROM rust
RUN apt-get update && apt-get install -y --no-install-recommends tini
COPY --from=builder /app/target/release/cube /cube
RUN ["/cube", "--version"]
ENTRYPOINT ["tini", "/cube", "--"]