# syntax=docker/dockerfile:1.4

### Builder ###
FROM lukemathwalker/cargo-chef:latest-rust-1.60.0 AS chef
WORKDIR /app

FROM chef AS planner
COPY --link . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS build
COPY --from=planner --link /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY --link . .
RUN cargo build --release

### Runner ###
FROM gcr.io/distroless/cc

COPY --from=build --link /app/target/release/idea-discussion-master /
USER nonroot

CMD ["./idea-discussion-master"]
