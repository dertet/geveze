FROM rust:1.51 as builder

RUN apt-get update && apt-get install -y musl-tools && rm -rf /var/lib/apt/lists/*
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/geveze

COPY . .

RUN cargo build --release --target x86_64-unknown-linux-musl

FROM debian

COPY --from=builder /usr/src/geveze/target/x86_64-unknown-linux-musl/release/geveze /geveze

CMD ["/geveze"]
