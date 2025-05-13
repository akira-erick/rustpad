FROM rust:1.70 as builder

WORKDIR /usr/src/actix_rustpad

COPY Cargo.toml Cargo.lock ./

RUN cargo fetch

COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/actix_rustpad

COPY --from=builder /usr/src/actix_rustpad/target/release/actix_rustpad .

EXPOSE 8080

CMD ["./actix_rustpad"]