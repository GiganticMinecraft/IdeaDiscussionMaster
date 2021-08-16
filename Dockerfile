FROM rust:1.54.0 AS builder
WORKDIR /app

RUN cargo build --release

FROM rust:1.54.0-alpine
WORKDIR /app

COPY --from=builder /app/target .

ENTRYPOINT ["/app/target/release/idea_discussion_master"]
