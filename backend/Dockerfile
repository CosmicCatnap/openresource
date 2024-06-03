# syntax=docker/dockerfile:1.4
FROM rust:bookworm AS base

ARG RUST_VERSION=1.78.0
ARG APP_NAME=openresource
FROM rust:${RUST_VERSION}-slim-bullseye AS build
ARG APP_NAME

ENV USER=root
WORKDIR /code
RUN cargo init
COPY Cargo.toml /code/Cargo.toml
RUN cargo fetch
COPY . /code

FROM base AS development

EXPOSE 8000

CMD [ "cargo", "run", "--offline" ]

FROM base AS dev-envs

EXPOSE 8000
RUN <<EOF
apt-get update
apt-get install -y --no-install-recommends git
EOF

RUN <<EOF
useradd -s /bin/bash -m vscode
groupadd docker
usermod -aG docker vscode
EOF

# install Docker tools (cli, buildx, compose)
COPY --from=gloursdocker/docker / /
CMD [ "cargo", "run", "--offline" ]

FROM base AS builder

# Build the application.
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=bind,source=migrations,target=migrations \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
ls -la
cargo build --locked --release
cp -r ./migrations /bin/migrations
cp ./target/release/$APP_NAME /bin/server
EOF

EXPOSE 8000

COPY --from=builder /code/target/release/openresource /bin/openresource

CMD [ "/bin/openresource" ]
