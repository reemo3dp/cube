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
RUN cargo build --release --target=x86_64-unknown-linux-musl

FROM alpine:latest
RUN apk update && apk add parallel
COPY --chmod=777 ./parallel.sh /parallel.sh
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/cube /cube
ENTRYPOINT ["/parallel.sh"]