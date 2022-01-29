FROM rust:1.58 as builder
WORKDIR /usr/src/truelayer-technical-pokemon
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.lock ./
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y openssl dumb-init && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/truelayer-technical-pokemon /usr/local/bin/truelayer-technical-pokemon
CMD ["dumb-init", "truelayer-technical-pokemon"]