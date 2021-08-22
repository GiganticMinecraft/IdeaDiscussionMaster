#TODO:  https://sysdig.jp/blog/dockerfile-best-practices/
# builder
FROM ekidd/rust-musl-builder:stable AS builder

## Build Cache Dependency Library
RUN mkdir /tmp/app
WORKDIR /tmp/app

## Build Dependency Library with DummyVersion.toml/lock
COPY --chown=rust:rust DummyVersion.toml ./Cargo.toml
COPY --chown=rust:rust DummyVersion.lock ./Cargo.lock
RUN mkdir -p src/ && touch src/lib.rs
RUN cargo build --release

## Build Base Library with Cargo.toml/lock
COPY --chown=rust:rust Cargo.toml ./Cargo.toml
COPY --chown=rust:rust Cargo.lock ./Cargo.lock
COPY --chown=rust:rust ./src/ ./src/
RUN cargo build --release && strip /tmp/app/target/x86_64-unknown-linux-musl/release/idea_discussion_master

# executor
FROM gcr.io/distroless/cc:latest
USER nonroot
WORKDIR /app
COPY --from=builder /tmp/app/target/x86_64-unknown-linux-musl/release/idea_discussion_master .

ENTRYPOINT ["/app/idea_discussion_master"]
