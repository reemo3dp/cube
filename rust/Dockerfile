FROM rust AS builder
WORKDIR /
RUN cargo new --bin app
WORKDIR /app
COPY Cargo.lock Cargo.toml ./
RUN cargo build --release && rm -Rf src/

COPY src ./src
ARG VERSION="0.0.0"
ENV VERSION=${VERSION}
RUN env && sed -i "s/version = \"0.0.0-dev\"/version = \"${VERSION}\"/" Cargo.toml
RUN touch src/main.rs && cargo build --release

RUN set -x; ! [ "$(./target/release/cube --version)" = "Hello, world!" ] 

FROM rust
RUN apt-get update && apt-get install -y --no-install-recommends tini
COPY --from=builder /app/target/release/cube /cube
RUN ["/cube", "--version"]
ENTRYPOINT ["tini", "/cube", "--"]