# This Dockerfile was modified from https://github.com/fly-apps/hello-rust

# Additionaly modification from https://loige.co/building_x86_rust-containers-from-mac-silicon/

# RESCUE:? https://stackoverflow.com/a/68117826

# -----------------
# --> Builder Image
# -----------------
FROM rust:slim-bookworm as builder

# Sets the working directory of the container on the host machine
WORKDIR /usr/src/app

# Dependency `ring` requires a cross-compiler for bundled C/C++
# sources, and may require Perl for some the target platforms.
RUN apt update && apt install -y --no-install-recommends \
  sudo \
  musl-tools \
  musl-dev \
  build-essential \
  gcc-x86-64-linux-gnu \
  clang \
  llvm \
  perl \
  curl \
  file

RUN update-ca-certificates

# Add our compilation target for musl
RUN rustup target add x86_64-unknown-linux-musl

# Install cargo-make to execute build scripts
RUN cargo install --locked cargo-make

# RUN PATH="/usr/local/cargo/bin:$PATH"

# Install the standalone executable for Tailwindcss
RUN \
  curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-arm64 && \
  chmod +x tailwindcss-linux-arm64 && \
  mv tailwindcss-linux-arm64 tailwindcss


# If the build fails for any reason we'll have some great backtraces!
ENV RUST_BACKTRACE=full
ENV CARGO_PROFILE_RELEASE_BUILD_OVERRIDE_DEBUG=true

# We have to tell Cargo to use the gcc linker, otherwise it will try to use cc
# and fail
# ENV RUSTFLAGS="-C linker=x86_64-linux-gnu-gcc"
# Tell Cargo which compoiler to use for building the `ring` dependency
# ENV CC_x86_64_unknown_linux_musl=clang
# Backup
# ENV AR_x86_64_unknown_linux_musl=llvm-ar
# Tell cargo to make this binary entirely statically linked
# ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_RUSTFLAGS="-Clink-self-contained=yes -Clinker=rust-lld"

# ~~~~~~~~~~~~~~~~~~
# ABSOLUTELY CRUCIAL
#
# For whatever reason, clang will look for C standard library headers in
# /usr/local/include/*.h
#
# Also for whatever reason, on Debian based docker images these files do not
# exist there.
#
# The solution is to copy them from their current directory to the expected
# directory.
#
# If you're reading this and thinking it's hacky, it is and I don't care
# enough about terrible C toolchain / compilers to fix it properly.
# ~~~~~~~~~~~~~~~~~~
RUN cd /usr/include/aarch64-linux-gnu && sudo cp -r . ..

# Copies everything from the local machine to the image
# Respects .dockerignore
COPY . .

# Will build and cache the binary and dependent crates in release mode
# This will cache cargo dependencies and only re-build if we add new
# dependencies
RUN \
  --mount=type=cache,target=target,rw \
  --mount=type=cache,target=/usr/local/cargo/registry,rw \
  makers release
# cargo build --release --target x86_64-unknown-linux-musl && \
# mv ./target/x86_64-unknown-linux-musl/release/rolo ./rolo

# Build the final tailwind CSS output


# -----------------
# --> Runtime Image
# -----------------
FROM alpine:3.19 as runtime

# Run as "app" user
# RUN useradd -ms /bin/bash app

RUN \
  addgroup --system appgroup && \
  adduser --system app --ingroup appgroup --home app

USER app
WORKDIR /app

# Get compiled binaries from builder's cargo install directory
COPY --from=builder /usr/src/app/rolo /app/rolo
# Get the static CSS assets from the builder (tailwindcss)
COPY --from=builder /usr/src/app/static /app/static

CMD ./rolo
