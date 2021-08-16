FROM rust:1.54.0 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM rust:1.54.0
WORKDIR /app
COPY --from=builder /app/target .

ENTRYPOINT ["/app/release/idea_discussion_master"]
