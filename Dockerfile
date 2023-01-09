  FROM lukemathwalker/cargo-chef:0.1.50-rust-buster AS chef
WORKDIR /app

FROM chef AS planner
COPY Cargo.* .
COPY api api
COPY core core
COPY ory ory
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY Cargo.* .
COPY api api
COPY core core
COPY ory ory
RUN cargo build --release --bin hub-identities-api


FROM debian:bullseye-slim as base
WORKDIR /app
RUN apt-get update -y && \
  apt-get install -y \
    ca-certificates \
    libpq5 \
    libssl1.1 \
  && \
  rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/hub-identities-api /usr/local/bin
ENTRYPOINT [ "/usr/local/bin/hub-identities-api" ]

