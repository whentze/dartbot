FROM rust:1.45.0 as builder

WORKDIR /usr/src/dartbot

COPY . .

RUN cargo build --release

FROM debian:slim

RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/dartbot/target/release/dartbot /usr/local/bin/dartbot

CMD ["dartbot"]
