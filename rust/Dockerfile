FROM rust AS builder
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR /app
COPY Cargo.lock Cargo.toml ./
RUN \
        mkdir -v src && \
        echo 'fn main() {}' > src/main.rs && \
        cargo build --release && \
        rm -Rvf src

COPY src src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends tini
COPY --from=builder /app/target/release/cube /cube
ENTRYPOINT ["tini", "/cube", "--"]