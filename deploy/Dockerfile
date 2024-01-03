# Shared arguments
ARG RUST_VERSION=1.74
ARG DEBIAN_NAME=bookworm


### Build image ###
# Rust build environment (compatible to release image)
FROM rust:$RUST_VERSION-slim-$DEBIAN_NAME AS build

# Individual directory for building
WORKDIR /api-snap

# Build the application & store results out of temporary cache directories
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=build.rs,target=build.rs \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=cache,target=/api-snap/target/ \
    --mount=type=cache,target=/api-snap/third-party/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --release --locked && \
    mkdir release && \
    cp -r target/release/api-snap third-party release/


### Release image ###
# Lightweight debian environment
FROM debian:$DEBIAN_NAME-slim AS release

# Copy build output from previous stage into this release stage
COPY --from=build /api-snap/release /api-snap

# Individual directory for running (& finding third-party directory)
WORKDIR /api-snap

# Configure the application for docker
ENV API_SNAP_ADDRESS=0.0.0.0
ENV API_SNAP_PORT=80

# Run the application on container start (further arguments can be passed by CMD)
ENTRYPOINT ["/api-snap/api-snap"]

# Expose port of running webserver
EXPOSE $API_SNAP_PORT

# Image metadata
ARG RUST_VERSION
ARG DEBIAN_NAME
LABEL rust_version=$RUST_VERSION \
    debian_name=$DEBIAN_NAME
