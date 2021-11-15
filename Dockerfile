FROM lukemathwalker/cargo-chef:0.1.32-rust-1.56 AS chef
WORKDIR /app

FROM chef AS planner
COPY ./Cargo.toml .
COPY ./Cargo.lock .
COPY ./src .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY ./Cargo.toml .
COPY ./Cargo.lock .
COPY ./src .
RUN cargo build --release

FROM debian:bullseye-slim
RUN groupadd -g 61000 docker
RUN useradd -g 61000 -l -m -s /bin/false -u 61000 docker
USER docker
WORKDIR /home/docker
COPY --from=builder --chown=docker:docker /app/target/release/idea-discussion-master .
ENTRYPOINT ["/home/docker/idea-discussion-master"]