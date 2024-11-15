FROM rust:1.82 AS builder
RUN apt-get update && \
    apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*
COPY . /usr/src/complete/server/
WORKDIR /usr/src/complete/server
RUN cargo build 


FROM debian:stable-slim
RUN apt-get update && \
    apt-get install -y build-essential \
    pkg-config \
    libssl-dev \
    curl \
    jq \
    && apt-get clean

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustc --version

COPY --from=builder /usr/src/complete /usr/local/bin/complete

EXPOSE 80 
WORKDIR /usr/local/bin/complete/server
ENV RUST_BACKTRACE=1

CMD cargo install sqlx-cli && \
    sqlx database create && \
    sqlx migrate run && \
    sh -c ./target/debug/server