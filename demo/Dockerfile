FROM rust:latest AS builder

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /target/release/demo /demo
RUN chmod +x /demo

EXPOSE 3000

CMD ["/demo"]
