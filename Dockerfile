FROM rust:1.45.0 as builder

WORKDIR /usr/src/dartbot

COPY . .

RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update && apt-get install -y libcurl4-openssl-dev libelf-dev libdw-dev binutils-dev && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/dartbot/target/release/dartbot /usr/local/bin/dartbot

CMD ["dartbot"]
