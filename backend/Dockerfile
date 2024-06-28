# syntax=docker/dockerfile:1.4

ARG APP_NAME=openresource

FROM rust:slim-bookworm AS base

FROM base as builder
RUN apt update
RUN apt install -y libpq-dev
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    --mount=type=bind,source=migrations,target=migrations \
    <<EOF
set -e
cargo build --locked --release
cp -r ./migrations /srv/migrations
cp ./target/release/openresource /srv/openresource
EOF
