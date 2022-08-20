FROM rust:1.63 AS builder

WORKDIR /app

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

COPY . .
RUN cargo build --bin rivet --release --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rivet .
USER 1000
CMD ["./rivet"]

