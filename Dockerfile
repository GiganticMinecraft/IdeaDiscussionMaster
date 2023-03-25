# syntax=docker/dockerfile:1.4
### Prepare for Runner ###
# chrono::LocalDate requires TimeZone file
FROM ubuntu:22.04 AS prepare-runner
RUN apt-get update \
    && apt-get install -y --no-install-recommends tzdata \
    && apt-get -y clean \
    && rm -rf /var/lib/apt/lists/*

### Builder ###
FROM clux/muslrust:1.68.1 AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY --link . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS build
COPY --from=planner --link /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

COPY --link . .
RUN cargo build --target x86_64-unknown-linux-musl --release

### Runner ###
FROM gcr.io/distroless/cc
USER nonroot
COPY --from=prepare-runner --link /usr/share/zoneinfo/Asia/Tokyo /etc/localtime
COPY --from=build --link /app/target/x86_64-unknown-linux-musl/release/c-presentation /idea-discussion-master

ENTRYPOINT ["/idea-discussion-master"]
