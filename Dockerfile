FROM rust:1.54.0

WORKDIR /app

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release

ENTRYPOINT ["/app/target/release/idea_discussion_master"]
