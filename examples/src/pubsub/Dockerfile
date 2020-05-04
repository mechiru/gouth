ARG RUST_VERSION=1.43
ARG DEBIAN_CODE_NAME=buster

FROM rust:${RUST_VERSION}-slim-${DEBIAN_CODE_NAME} AS builder
WORKDIR /workdir
COPY ./ ./
RUN cargo build -p examples --release

FROM debian:${DEBIAN_CODE_NAME}-slim AS runtime
WORKDIR /app
COPY --from=builder /workdir/target/release/pubsub /app/pubsub
RUN groupadd rust && \
  useradd -g rust rust && \
  chown rust:rust /app
USER rust
ENTRYPOINT ["./pubsub"]
