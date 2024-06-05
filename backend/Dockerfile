ARG RUST_VERSION=1.72.0
ENV DATABASE_URL=postgres://openresource:thisisabadpassword@localhost:5432/openresource
FROM rust:bookworm AS build
WORKDIR /code
COPY Cargo.toml . 
COPY Cargo.lock . 
RUN --mount=type=bind,source=src,target=src,z \
    --mount=type=bind,source=migrations,target=migrations,z \
    --mount=type=cache,target=/code/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
cargo build --locked --release
cp -r ./migrations /srv/migrations
cp ./target/release/openresource /srv/server
EOF

FROM rust:bookworm AS final
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid 10001 \
    appuser
USER appuser

# Copy the executable from the "build" stage.
COPY --from=build /srv/server /srv/server

# Expose the port that the codelication listens on.
EXPOSE 3000

# What the container should run when it is started.
CMD ["/srv/server"]
