# syntax=docker/dockerfile:1

################################################################################
# Create a stage for building the application.

ARG RUST_VERSION=1.78.0
ARG APP_NAME=openresource
FROM rust:${RUST_VERSION}-slim-bullseye AS build
ARG APP_NAME

WORKDIR /app

RUN apt update
RUN apt install -y libpq-dev



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

# base image
FROM debian:bookworm-slim AS final

# non-privileged user that the app will run under.
# https://docs.docker.com/develop/develop-images/dockerfile_best-practices/#user
ARG UID=10001
RUN apt update
RUN apt install -y libpq-dev

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser



# Copy the executable from the "build" stage.
COPY --from=build /bin/server /bin/

# Expose application port.
EXPOSE 3123

# Rust Bin
CMD ["/bin/server"]
