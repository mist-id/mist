FROM rust:latest AS builder

COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y debian-keyring debian-archive-keyring apt-transport-https curl
RUN curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/gpg.key' | gpg --dearmor -o /usr/share/keyrings/caddy-stable-archive-keyring.gpg
RUN curl -1sLf 'https://dl.cloudsmith.io/public/caddy/stable/debian.deb.txt' | tee /etc/apt/sources.list.d/caddy-stable.list

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata caddy \
    && rm -rf /var/lib/apt/lists/*

COPY scripts/start.sh start.sh
COPY Caddyfile Caddyfile
COPY --from=builder /target/release/demo /demo
RUN chmod +x /demo /start.sh

EXPOSE 80 443 3000

CMD ["/start.sh"]
