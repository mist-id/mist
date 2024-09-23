FROM rust:latest AS builder

COPY . .
RUN cargo build --release --package mist_runner

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /target/release/mist_runner /mist
RUN chmod +x /mist

RUN useradd -m casper
USER casper
EXPOSE 9001 9002

CMD ["/mist"]
