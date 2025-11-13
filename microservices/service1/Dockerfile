FROM rust:1.91.0-slim-bullseye AS builder
RUN apt-get update && apt-get install -y --no-install-recommends libpq-dev
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends libpq5 curl build-essential postgresql-client
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/service1 /app/
COPY .env /app/.env
COPY Rocket.toml /app/Rocket.toml
COPY migrations /app/migrations
EXPOSE 8000
ENV ROCKET_CONFIG=/app/Rocket.toml
CMD ["./service1"]