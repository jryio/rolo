# This Dockerfile was modified from https://github.com/fly-apps/hello-rust
# Additionaly modification from https://loige.co/building_x86_rust-containers-from-mac-silicon/
# RESCUE:? https://stackoverflow.com/a/68117826
# Also SQLITE LINKING RESCUE? https://github.com/rust-lang/rust/issues/115430

# -----------------
# --> Builder Image
# -----------------
FROM rust:bookworm as builder

# Sets the working directory of the container on the host machine
WORKDIR /usr/src/app

RUN apt update && apt install -y --no-install-recommends curl

RUN update-ca-certificates

# Install cargo-make to execute build scripts
RUN \
  --mount=type=cache,target=/usr/local/cargo,from=rust:bookworm,source=/usr/local/cargo \
  cargo install cargo-make

# Install the standalone executable for Tailwindcss
RUN \
  curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-arm64 && \
  chmod +x tailwindcss-linux-arm64 && \
  mv tailwindcss-linux-arm64 tailwindcss

# Copies everything from the local machine to the image
# Respects .dockerignore
COPY . .

ENV RUSTFLAGS="-Ctarget-feature=+crt-static"
ENV RUST_BACKTRACE=full

# Will build and cache the binary and dependent crates in release mode
# This will cache cargo dependencies and only re-build if we add new
# dependencies
RUN \
  --mount=type=cache,target=/usr/local/cargo,from=rust:bookworm,source=/usr/local/cargo \
  --mount=type=cache,target=target,rw \
  --mount=type=cache,target=/usr/local/cargo/registry,rw \
  makers release

# -----------------
# --> Runtime Image
# -----------------
FROM gcr.io/distroless/cc-debian12 as runtime

WORKDIR /app

# Get compiled binaries from builder's cargo install directory
COPY --from=builder /usr/src/app/rolo /app/rolo
# Get the static CSS assets from the builder (tailwindcss)
COPY --from=builder /usr/src/app/static /app/static

CMD ["./rolo"]
