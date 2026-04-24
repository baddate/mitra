# syntax=docker/dockerfile:1.4
#
# Multi-stage Dockerfile for Mitra
# Build context must be the project root:
#   docker build -f contrib/docker/Dockerfile -t mitra .
#
# Stages:
#   frontend-build  – clones and builds the mitra-web SPA
#   chef            – base image with cargo-chef installed (shared)
#   planner         – analyses Cargo workspace, emits recipe.json
#   builder         – cooks deps (cached), then builds the Rust binary
#   runtime         – minimal Alpine image that ships to production

# -----------------------------------------------------------------------------
# Stage 1 – Frontend (Node)
# Clone mitra-web and produce a static dist/ bundle.
# -----------------------------------------------------------------------------
FROM node:24-alpine AS frontend-build

RUN apk add --no-cache git

WORKDIR /app

# Allow overriding the frontend repo and branch at build time
ARG FRONTEND_REPO=https://codeberg.org/silverpill/mitra-web.git
ARG FRONTEND_BRANCH=main

RUN git clone --depth 1 --branch "$FRONTEND_BRANCH" "$FRONTEND_REPO" .

ARG VITE_BACKEND_URL
ENV VITE_BACKEND_URL=$VITE_BACKEND_URL

# npm ci is faster and strictly reproducible compared to npm install
RUN npm ci --prefer-offline \
 && npm run build

# -----------------------------------------------------------------------------
# Stage 2 – cargo-chef base
# Install cargo-chef once; reused by both planner and builder stages so the
# installation layer is cached and never re-downloaded.
# -----------------------------------------------------------------------------
FROM rust:alpine AS chef

# musl-dev provides the C headers needed by some *-sys crates under musl libc
RUN apk add --no-cache musl-dev \
 && cargo install cargo-chef --locked

WORKDIR /app

# -----------------------------------------------------------------------------
# Stage 3 – Planner
# Inspect Cargo.toml / Cargo.lock and produce a build "recipe" that captures
# the exact set of dependencies without needing the full source tree.
# This stage is invalidated only when Cargo.toml or Cargo.lock change.
# -----------------------------------------------------------------------------
FROM chef AS planner

# For a Cargo workspace, cargo metadata needs the full source tree to resolve
# all workspace members. Copy everything; prepare itself is nearly instant.
COPY . .

# If this is a Cargo workspace, uncomment and adjust the line below:
# COPY crates/ crates/

RUN cargo chef prepare --recipe-path recipe.json


# -----------------------------------------------------------------------------
# Stage 4 – Builder
# Two-step compilation:
#   1. `cargo chef cook`  – compiles dependencies only (cached when recipe
#                           is unchanged, even if src/ changes)
#   2. `cargo build`      – compiles the application itself
# -----------------------------------------------------------------------------
FROM chef AS builder

# Restore the dependency recipe from the planner stage
COPY --from=planner /app/recipe.json recipe.json

# Cook (compile) dependencies. This layer is re-used on every subsequent build
# as long as recipe.json stays the same, saving several minutes per iteration.
RUN cargo chef cook --release --features production --recipe-path recipe.json

# Now copy the full source. Only layers from here onward are invalidated by
# application source changes.
COPY . .

# Build the final binary
RUN cargo build --release --features production

# -----------------------------------------------------------------------------
# Stage 5 – Runtime
# Minimal Alpine image: only the binary, frontend assets, and ca-certificates.
# No compiler, no Rust toolchain, no build artefacts.
# -----------------------------------------------------------------------------
FROM alpine:3.22

# ca-certificates: required for outgoing TLS connections (e.g. federation)
RUN apk add --no-cache ca-certificates

# Create a dedicated non-root user for the service
RUN addgroup -S mitra \
 && adduser  -S -G mitra mitra

# Prepare runtime directories
RUN mkdir -p \
      /var/lib/mitra \
      /etc/mitra \
      /usr/share/mitra/www \
 && chown -R mitra:mitra /var/lib/mitra /etc/mitra

# Frontend static assets
COPY --from=frontend-build /app/dist /usr/share/mitra/www/

# Application binary
COPY --from=builder /app/target/release/mitra /usr/bin/mitra

USER mitra

# Default config path; supports both yaml and toml formats.
# Override at runtime with -e CONFIG_PATH=/path/to/config.toml
ENV CONFIG_PATH=/etc/mitra/config.yaml

# Expose the default HTTP port (override in config if needed)
EXPOSE 8080

CMD ["mitra", "server"]
