FROM rust:1.86.0-slim as base
RUN cargo install cargo-chef
FROM base as planner
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM base as builder
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN apt update && apt install -y pkg-config libglib2.0-dev libgdk-pixbuf-2.0-0 libcairo2 libpango-1.0-0 libgtk-4-media-gstreamer
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build
