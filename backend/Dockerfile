ARG RUST_VERSION=1.72.0

FROM rust:bookworm AS build
WORKDIR /build
COPY .env .
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    --mount=type=bind,source=migrations,target=migrations \
    <<EOF
	set -e
	cargo build --locked --release
	mkdir -p /srv/app
	cp -r ./migrations /srv/app/migrations
	cp .env /srv/app/.env
	cp ./target/release/openresource /srv/app/server
EOF

FROM rust:bookworm AS final
RUN mkdir -p /srv/app
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/srv/app" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid 10001 \
    appuser

# Copy and configure executable from build
COPY --from=build /srv/app /srv/app
RUN chown -R appuser:appuser /srv/app
RUN chmod +x /srv/app/server

# Expose the application port
EXPOSE 8000

# Run backend
USER appuser
CMD ["/srv/app/server"]

