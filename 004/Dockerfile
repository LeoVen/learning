FROM rust:1.85 AS chef

RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

ARG BINARY_NAME
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin ${BINARY_NAME}

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime

RUN apt-get update -y
RUN apt-get install -y openssl ca-certificates

ARG BINARY_NAME
WORKDIR /app

COPY ./config/${BINARY_NAME}.conf.toml ./config/${BINARY_NAME}.conf.toml
COPY ./public ./public

COPY --from=builder /app/target/release/${BINARY_NAME} ./rust_program

ENTRYPOINT ["/app/rust_program"]
