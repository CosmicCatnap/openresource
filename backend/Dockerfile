ARG APP_NAME=openresource

FROM rust:slim-bookworm AS build
ARG APP_NAME
WORKDIR /code

# add postgres required lib
RUN apt update
RUN apt-get update && apt-get install -y libpq-dev

SHELL ["/bin/bash", "-c"]
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/code/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    --mount=type=bind,source=migrations,target=migrations \
    cargo build --locked --release &&\
    cp ./target/release/$APP_NAME /bin/server

# run backend
FROM debian:bookworm AS final
RUN apt-get update && apt-get install -y libpq5 libpq-dev
# build and set appuser
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

# copy and run exec
COPY --from=build /bin/server /bin/
EXPOSE 8000
CMD ["/bin/server"]