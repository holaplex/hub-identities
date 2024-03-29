FROM rust:1.69.0-bullseye as chef
RUN cargo install cargo-chef --locked

WORKDIR /app

FROM chef AS planner
COPY Cargo.* .
COPY api api
COPY ory ory
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY Cargo.* .
COPY api api
COPY ory ory
RUN cargo build --release --bin holaplex-hub-identities


FROM debian:bullseye-slim as base
WORKDIR /app
RUN apt-get update -y && \
  apt-get install -y --no-install-recommends \
  ca-certificates \
  libpq5 \
  libssl1.1 \
  && \
  rm -rf /var/lib/apt/lists/*

ENV TZ=Etc/UTC
ENV APP_USER=runner

RUN groupadd $APP_USER \
    && useradd --uid 10000 -g $APP_USER $APP_USER \
    && mkdir -p bin

RUN chown -R $APP_USER:$APP_USER bin

USER 10000

COPY --from=builder /app/target/release/holaplex-hub-identities /usr/local/bin
ENTRYPOINT ["/usr/local/bin/holaplex-hub-identities"]
