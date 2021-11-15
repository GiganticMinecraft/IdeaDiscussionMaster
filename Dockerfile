FROM lukemathwalker/cargo-chef:0.1.32-rust-1.56 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc:latest
USER nonroot
WORKDIR /app
COPY --from=builder /app/target/release/idea-discussion-master .
ENTRYPOINT ["/app/idea-discussion-master"]