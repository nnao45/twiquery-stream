FROM rust:1.34.1-slim as builder
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    OPENSSL_LIB_DIR=/usr/lib/x86_64-linux-gnu \
    OPENSSL_INCLUDE_DIR=/usr/include/openssl \
    RUST_VERSION=%%RUST-VERSION%%
RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
        git \
        libssl-dev
RUN git clone https://github.com/nnao45/twiquery-batch
WORKDIR /twiquery-batch
RUN cargo build --release

FROM debian:9.9-slim
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y \
        ca-certificates \
        tzdata
RUN cp /usr/share/zoneinfo/Asia/Tokyo /etc/localtime && \
    echo "Asia/Tokyo" > /etc/timezone
COPY --from=builder /twiquery-batch/target/release/twiquery-batch .
CMD ["./twiquery-batch"]