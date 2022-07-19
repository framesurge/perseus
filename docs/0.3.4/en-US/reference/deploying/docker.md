# Docker Deployment

For situations where [serverful deployment](:reference/deploying/serverful) is required, or in case there is a need to deploy one of the examples found on GitHub without prior setup of all necessary dependencies, below are `Dockerfile` examples meant to serve for different deployment scenarios. These steps can also serve as guidelines for production deployments.

Note that the following examples should be modified for your particular use-case rather than being used as-is. Also, these `Dockerfile`s are standalone because they use `curl` to download examples directly from the Perseus repository (of course, you'll probably want to use your own code in production).

Before proceeding with this section, you should be familiar with Docker's [multi-stage builds system](https://docs.docker.com/develop/develop-images/multistage-build) and Perseus' [code size optimizations](:reference/deploying/size).

<details>
<summary>Production example using the size optimizations plugin</summary>

```dockerfile
# Pull base image.
FROM rust:1.57-slim AS base

# Define optional command-line build arguments.
ARG PERSEUS_VERSION \
  PERSEUS_SIZE_OPT_VERSION \
  SYCAMORE_VERSION \
  BINARYEN_VERSION \
  ESBUILD_VERSION \
  ESBUILD_TARGET \
  WASM_PACK_VERSION \
  RUST_RELEASE_CHANNEL \
  CARGO_NET_GIT_FETCH_WITH_CLI

# Export environment variables.
ENV PERSEUS_VERSION=${PERSEUS_VERSION:-0.3.5} \
  PERSEUS_SIZE_OPT_VERSION=${PERSEUS_SIZE_OPT_VERSION:-0.1.9} \
  SYCAMORE_VERSION=${SYCAMORE_VERSION:-0.7.1} \
  BINARYEN_VERSION=${BINARYEN_VERSION:-104} \
  ESBUILD_VERSION=${ESBUILD_VERSION:-0.14.7} \
  ESBUILD_TARGET=${ESBUILD_TARGET:-es6} \
  WASM_PACK_VERSION=${WASM_PACK_VERSION:-0.10.3} \
  RUST_RELEASE_CHANNEL=${RUST_RELEASE_CHANNEL:-stable} \
  CARGO_NET_GIT_FETCH_WITH_CLI=${CARGO_NET_GIT_FETCH_WITH_CLI:-false}

# Work from the root of the container.
WORKDIR /

# Install build dependencies.
RUN apt-get update \
  && apt-get -y install --no-install-recommends \
  apt-transport-https \
  build-essential \
  curl \
  gawk \
  git \
  libssl-dev \
  lsb-release \
  openssl \
  pkg-config \
  && rustup install ${RUST_RELEASE_CHANNEL} \
  && rustup default ${RUST_RELEASE_CHANNEL} \
  && rustup target add wasm32-unknown-unknown

# Create a build stage for `binaryen` we can run in parallel.
FROM base as binaryen

# Work from the chosen install path for `binaryen`.
WORKDIR /binaryen

# Download, extract, and remove compressed tar of `binaryen`.
RUN curl -L#o binaryen-${BINARYEN_VERSION}.tar.gz \
  https://github.com/WebAssembly/binaryen/releases/download/version_${BINARYEN_VERSION}/binaryen-version_${BINARYEN_VERSION}-x86_64-linux.tar.gz \
  && tar --strip-components=1 -xzf binaryen-${BINARYEN_VERSION}.tar.gz \
  && rm -f binaryen-${BINARYEN_VERSION}.tar.gz

# Create a build stage for `esbuild` we can run in parallel.
FROM base as esbuild

# Work from the chosen install path for `esbuild`.
WORKDIR /esbuild

# Download, extract, and remove compressed tar of `esbuild`.
RUN curl -L#o esbuild-${ESBUILD_VERSION}.tar.gz \
  https://registry.npmjs.org/esbuild-linux-64/-/esbuild-linux-64-${ESBUILD_VERSION}.tgz \
  && tar --strip-components=1 -xzf esbuild-${ESBUILD_VERSION}.tar.gz \
  && rm -f esbuild-${ESBUILD_VERSION}.tar.gz

# Create a build stage for `perseus-size-opt` that we can run in parallel.
FROM base as perseus-size-opt

# Work from the chosen install path for `perseus-size-opt`.
WORKDIR /perseus-size-opt

# Download and make adjustments to the source of `perseus-size-opt`.
RUN curl -L# https://codeload.github.com/arctic-hen7/perseus-size-opt/tar.gz/v${PERSEUS_SIZE_OPT_VERSION} \
  | tar -xz --strip-components=1 \
  && rm -rf ./examples \
  && sed -i "s|^\(perseus =\).*$|\1 { path = \\\"/perseus/packages/perseus\\\" }|;" ./Cargo.toml \
  && printf '%s\n' \
  '#!/bin/sh' \
  'rm_workspace () {' \
  '  local a=$( grep -ne "^\[workspace\]$" ./Cargo.toml | grep -Eo "^[^:]+" )' \
  '  local b=$( grep -ne "^\]$" ./Cargo.toml | grep -Eo "^[^:]+" )' \
  '  sed -i "${a},${b}d" ./Cargo.toml' \
  '}' \
  'rm_workspace' > ./rm_workspace.sh \
  && chmod +x ./rm_workspace.sh && . ./rm_workspace.sh && rm -f ./rm_workspace.sh

# Create a build stage for `wasm-pack` that we can run in parallel.
FROM base as wasm-pack

# Work from the chosen install path for `wasm-pack`.
WORKDIR /wasm-pack

# Install crate `wasm-pack` into the work path.
RUN cargo install wasm-pack --version $WASM_PACK_VERSION \
  && mv /usr/local/cargo/bin/wasm-pack .

# Create a build stage for the codebase of the `perseus` framework that we can run in parallel.
FROM base as framework

# Work from the root of the codebase.
WORKDIR /perseus

# Download and make adjustments to the codebase of the framework.
RUN curl -L# https://codeload.github.com/arctic-hen7/perseus/tar.gz/v${PERSEUS_VERSION} \
  | tar -xz --strip-components=1 \
  && sed -i "\
  s|\(println!.*\)$|// \1|; \
  s|\.\./\.\.examples/core/basic/\.perseus|/perseus/examples/core/basic/\.perseus|g; \
  s|\(fs::remove_dir_all(dest\.join(\"\.perseus/dist\"))\.unwrap();\)|// \1|; \
  s|PERSEUS_VERSION|path = \\\\\"/perseus/packages/perseus\\\\\"|g; \
  s|PERSEUS_ACTIX_WEB_VERSION|path = \\\\\"/perseus/packages/perseus-actix-web\\\\\"|g; \
  s|PERSEUS_WARP_VERSION|path = \\\\\"/perseus/packages/perseus-warp\\\\\"|g;" \
  ./packages/perseus-cli/build.rs \
  && sed -i "\
  s|^\(perseus = {\) \(.*\) \(}\)$|\1 \2, features = \[ \"hydrate\" \] \3|; \
  s|^\(sycamore =\).*$|\1 \"=${SYCAMORE_VERSION}\"|; \
  " ./examples/comprehensive/tiny/Cargo.toml \
  && printf '%s\n' \
  'perseus-size-opt = { path = "/perseus-size-opt" }' >> ./examples/comprehensive/tiny/Cargo.toml \
  && rm -f ./examples/comprehensive/tiny/src/lib.rs \
  && printf '%s\n' \
  'use perseus::{ErrorPages, Html, PerseusApp, Plugins, Template};' \
  'use sycamore::view;' \
  '' \
  'use perseus_size_opt::{perseus_size_opt, SizeOpts};' \
  '' \
  '#[perseus::main]' \
  'pub fn main<G: Html>() -> PerseusApp<G> {' \
  '    PerseusApp::new()' \
  '        .template(|| {' \
  '            Template::new("index").template(|_| {' \
  '                view! {' \
  '                    p { "Hello World!" }' \
  '                }' \
  '            })' \
  '        })' \
  '        .error_pages(|| ErrorPages::new(|url, status, err, _| {' \
  '            view! {' \
  "                p { (format!(\"An error with HTTP code {} occurred at '{}': '{}'.\", status, url, err)) }" \
  '            }' \
  '        }))' \
  '        .plugins(' \
  '            Plugins::new()' \
  '                .plugin(' \
  '                    perseus_size_opt,' \
  '                    SizeOpts {' \
  '                        wee_alloc: true,' \
  '                        lto: true,' \
  '                        opt_level: "s".to_string(),' \
  '                        codegen_units: 1,' \
  '                        enable_fluent_bundle_patch: false,' \
  '                    }' \
  '                )' \
  '        )' \
  '}' > ./examples/comprehensive/tiny/src/lib.rs \
  && sed -i "s|^\(sycamore =\).*$|\1 \"=${SYCAMORE_VERSION}\"|" \
  ./examples/core/basic/Cargo.toml \
  && sed -i "\
  s|^\(sycamore =\).*$|\1 { version = \\\"=${SYCAMORE_VERSION}\\\", features = [ \\\"ssr\\\" ] }|;
  s|^\(sycamore-router =\).*$|\1 \\\"=${SYCAMORE_VERSION}\\\"|;" \
  ./examples/core/basic/.perseus/Cargo.toml

# Create a build stage for `perseus-cli` that we can run in parallel.
FROM framework as perseus-cli

# Copy perseus-size-opt to satisfy dependencies.
COPY --from=perseus-size-opt /perseus-size-opt/ /perseus-size-opt/

# Work from the root of the package.
WORKDIR /perseus/packages/perseus-cli

# Compile the release profile of package `perseus-cli`.
RUN cargo build --bin perseus --release

# Create a build stage for building our app.
FROM framework as builder

# Copy the tools we previously prepared in parallel.
COPY --from=binaryen /binaryen/bin/ /usr/bin/
COPY --from=binaryen /binaryen/include/ /usr/include/
COPY --from=binaryen /binaryen/lib/ /usr/lib/
COPY --from=esbuild /esbuild/bin/esbuild /usr/bin/
COPY --from=perseus-cli /perseus/target/release/perseus /usr/bin/
COPY --from=perseus-size-opt /perseus-size-opt/ /perseus-size-opt/
COPY --from=wasm-pack /wasm-pack/wasm-pack /usr/bin/

# Work from the root of our app.
WORKDIR /perseus/examples/comprehensive/tiny

# Execute all necessary commands for deploying our app.
RUN . /etc/profile && . /usr/local/cargo/env \
  && perseus clean \
  && perseus prep \
  && perseus tinker \
  && printf '%s\n' \
  '#!/bin/sh' \
  'parse_file () {' \
  '  local file_path=./.perseus/src/lib.rs' \
  '  local line_num=$( grep -ne "clippy" $file_path | grep -Eo "^[^:]+" )' \
  '  if [ ! -z "${line_num}" ] && [ $line_num -ne 1 ]' \
  '  then' \
  '    awk -i inplace '"\\" \
  '    -v line_num=$line_num '"\\" \
  '    -v inner_attr=$( sed "${line_num}q;d" $file_path | cut -d " " -f 1 ) '"\\" \
  '    "NR==1 { print inner_attr } NR!=line_num { print }" $file_path' \
  '  fi' \
  '}' \
  'parse_file' > ./parse_file.sh \
  && chmod +x ./parse_file.sh && . ./parse_file.sh && rm -f ./parse_file.sh \
  && export PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig \
  && perseus deploy \
  && esbuild ./pkg/dist/pkg/perseus_engine.js \
  --minify \
  --target=${ESBUILD_TARGET} \
  --outfile=./pkg/dist/pkg/perseus_engine.js \
  --allow-overwrite \
  && wasm-opt \
  -Os ./pkg/dist/pkg/perseus_engine_bg.wasm \
  -o ./pkg/dist/pkg/perseus_engine_bg.wasm

# Prepare the final image where the app will be deployed.
FROM debian:stable-slim

# Work from a chosen install path for the deployed app.
WORKDIR /app

# Copy the app into its chosen install path.
COPY --from=builder /perseus/examples/comprehensive/tiny/pkg /app/

# Bind the server to `localhost`.
ENV PERSEUS_HOST=0.0.0.0

# Bind the container to the default port of 8080.
ENV PERSEUS_PORT=8080

# Configure the container to automatically serve the deployed app while running.
CMD ["./server"]
```

</details>

<details>
<summary>Production examples using `wee_alloc` manually</summary>

```dockerfile
# Pull the base image.
FROM rust:1.57-slim AS build

# Install build dependencies.
RUN apt update \
  && apt -y install --no-install-recommends \
  apt-transport-https \
  build-essential \
  curl \
  libssl-dev \
  lsb-release \
  openssl \
  pkg-config

# Export environment variables.
ENV PERSEUS_VERSION=0.3.5 \
  SYCAMORE_VERSION=0.7.1 \
  WEE_ALLOC_VERSION=0.4.5 \
  BINARYEN_VERSION=104 \
  ESBUILD_VERSION=0.14.7 \
  WASM_PACK_VERSION=0.10.3 \
  RUST_RELEASE_CHANNEL=stable

# Work from the root of the project.
WORKDIR /app

# Perform the following steps:
# - Install latest `rust` from `stable` release channel.
# - Set `rust:stable` as default toolchain.
# - Download the target for `wasm`.
RUN rustup install $RUST_RELEASE_CHANNEL \
  && rustup default $RUST_RELEASE_CHANNEL \
  && target add wasm32-unknown-unknown

# Install crate `perseus-cli`
RUN cargo install perseus-cli --version $PERSEUS_VERSION

# Install crate `wasm-pack`.
RUN cargo install wasm-pack --version $WASM_PACK_VERSION

# Retrieve the src of the project and remove unnecessary boilerplate.
RUN curl -L# \
  https://codeload.github.com/arctic-hen7/perseus/tar.gz/v${PERSEUS_VERSION} \
  | tar -xz --strip=3 perseus-${PERSEUS_VERSION}/examples/comprehensive/tiny

# Download, unpack, symlink, and verify install of `binaryen`.
RUN curl -L#o binaryen-${BINARYEN_VERSION}.tar.gz \
  https://github.com/WebAssembly/binaryen/releases/download/version_${BINARYEN_VERSION}/binaryen-version_${BINARYEN_VERSION}-x86_64-linux.tar.gz \
  && tar -xzf binaryen-${BINARYEN_VERSION}.tar.gz \
  && ln -s $(pwd)/BINARYEN_Version_${BINARYEN_VERSION}/bin/wasm-opt /usr/bin/wasm-opt \
  && wasm-opt --version

# Download, unpack, symlink, and verify install of `esbuild`.
RUN curl -L#o esbuild-${ESBUILD_VERSION}.tar.gz \
  https://registry.npmjs.org/esbuild-linux-64/-/esbuild-linux-64-${ESBUILD_VERSION}.tgz \
  && tar -xzf esbuild-${ESBUILD_VERSION}.tar.gz \
  && ln -s $(pwd)/package/bin/esbuild /usr/bin/esbuild \
  && esbuild --version

# Work from the src of the project.
WORKDIR /app/tiny

# Specify precise dependency versions in the project's `Cargo.toml` file.
RUN sed -i "\
  s|^\(perseus =\).*$|\1 \"=${PERSEUS_VERSION}\"|; \
  s|^\(sycamore =\).*$|\1 \"=${SYCAMORE_VERSION}\"|;
  s|^\(\[dependencies\]\)$|\1\nwee_alloc = \"=${WEE_ALLOC_VERSION}\"|;" \
  ./Cargo.toml && cat ./Cargo.toml

# Prepend modifications to the src of the project to implement `wee_alloc` in `lib.rs`.
RUN sed -i "1i \
  #[global_allocator]\n\
  static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;\n" \
  ./src/lib.rs && cat ./src/lib.rs

# Update dependencies to their precise, required versions.
RUN cargo update -p perseus --precise $PERSEUS_VERSION \
  && cargo update -p sycamore --precise $SYCAMORE_VERSION \
  && cargo update -p wee_alloc --precise $WEE_ALLOC_VERSION

# Clean any pre-existing generated `./perseus` subdirectory from the project,
# then prepare the project prior to ejecting it from the CLI.
RUN perseus clean \
  && perseus prep \
  && perseus eject

# Append necessary modifications to the `Cargo.toml` file in the prepared project.
RUN sed -i "s|^\(perseus =\).*$|\1 \"${PERSEUS_VERSION}\"|g" .perseus/Cargo.toml \
  && printf '%s\n' \
  "" "" \
  "[profile.release]" \
  "codegen-units = 1" \
  "opt-level = \"s\"" \
  "lto = true" >> .perseus/Cargo.toml \
  && cat .perseus/Cargo.toml

# Patch `clippy` inner attribute syntax error in `lib.rs` (if found).
RUN sed -i "s|\(#\)!\(\[allow(clippy::unused_unit)\]\)|\1\2|;" ./.perseus/src/lib.rs

# Single-threaded perseus CLI mode required for low memory environments.
# ENV PERSEUS_CLI_SEQUENTIAL=true

# Export variables required by `wasm-bindgen` and deploy the app from the project.
RUN export PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig \
  && perseus deploy

# Run `esbuild` against `bundle.js` to optimize it into minified format.
RUN esbuild ./pkg/dist/pkg/perseus_engine.js \
  --minify \
  --target=esnext \
  --outfile=./pkg/dist/pkg/perseus_engine.js \
  --allow-overwrite \
  && ls -lha ./pkg/dist/pkg

# Run `wasm-opt` against `bundle.wasm` to optimize it based on bytesize.
RUN wasm-opt \
  -Os ./pkg/dist/pkg/perseus_engine_bg.wasm \
  -o ./pkg/dist/pkg/perseus_engine_bg.wasm \
  && ls -lha ./pkg/dist/pkg

# Prepare the final image where the app will be deployed.
FROM debian:stable-slim

# Work from a chosen install path for the deployed app.
WORKDIR /app

# Copy the app into its chosen install path.
COPY --from=build /app/tiny/pkg /app/

# Bind the server to `localhost`.
ENV HOST=0.0.0.0

# Bind the container to the default port of 8080.
ENV PORT=8080

# Configure the container to automatically serve the deployed app while running.
CMD ["./server"]
```

</details>

<details>
<summary>Test example for deploying a specific branch from the Perseus repository</summary>

```dockerfile
# Pull base image.
FROM rust:1.57-slim AS build

# Install build dependencies.
RUN apt update \
  && apt -y install --no-install-recommends \
  apt-transport-https \
  build-essential \
  curl \
  libssl-dev \
  lsb-release \
  nodejs \
  npm \
  openssl \
  pkg-config

# Export environment variables.
ENV PERSEUS_BRANCH=main \
  EXAMPLE_CATEGORY=comprehensive \
  EXAMPLE_NAME=tiny \
  BONNIE_VERSION=0.3.2 \
  BINARYEN_VERSION=104 \
  ESBUILD_VERSION=0.14.7 \
  WASM_PACK_VERSION=0.10.3 \
  RUST_RELEASE_CHANNEL=stable

# Work from the root of the project.
WORKDIR /app

# Download the target for `wasm`.
RUN rustup install $RUST_RELEASE_CHANNEL \
  && rustup default $RUST_RELEASE_CHANNEL \
  && rustup target add wasm32-unknown-unknown

# Install crate `bonnie`.
RUN cargo install bonnie --version $BONNIE_VERSION

# Install crate `wasm-pack`.
RUN cargo install wasm-pack --version $WASM_PACK_VERSION

# Install dependencies required by package `perseus-website`.
RUN npm i -g browser-sync concurrently serve tailwindcss

# Download, unpack, symlink, and verify install of `binaryen`.
RUN curl -L#o binaryen-${BINARYEN_VERSION}.tar.gz \
  https://github.com/WebAssembly/binaryen/releases/download/version_${BINARYEN_VERSION}/binaryen-version_${BINARYEN_VERSION}-x86_64-linux.tar.gz \
  && tar -xzf binaryen-${BINARYEN_VERSION}.tar.gz \
  && ln -s $(pwd)/BINARYEN_Version_${BINARYEN_VERSION}/bin/wasm-opt /usr/bin/wasm-opt \
  && wasm-opt --version

# Download, unpack, symlink, and verify install of `esbuild`.
RUN curl -L#o esbuild-${ESBUILD_VERSION}.tar.gz \
  https://registry.npmjs.org/esbuild-linux-64/-/esbuild-linux-64-${ESBUILD_VERSION}.tgz \
  && tar -xzf esbuild-${ESBUILD_VERSION}.tar.gz \
  && ln -s $(pwd)/package/bin/esbuild /usr/bin/esbuild \
  && esbuild --version

# Retrieve the current state of a branch in the `perseus` repo.
RUN curl -L# \
  https://codeload.github.com/arctic-hen7/perseus/tar.gz/${PERSEUS_BRANCH} \
  | tar -xz

# Work from the requested branch of `perseus`.
WORKDIR /app/perseus-${PERSEUS_BRANCH}

# Perform the following steps:
# - Patch `bonnie.toml` to remove backticks.
#   - These break echoed strings and cause builds to fail.
# - Instruct `cargo` to only compile the binary target `perseus`.
#   - Prevents "no space left on device" error in `docker`.
RUN sed -i "\
  s|\(cargo build\)|\1 --bin perseus|; \
  s|\`|'|g" ./bonnie.toml

# Compile and install `perseus-cli` as defined by the current state of the repo's branch.
RUN bonnie setup

# Clean any pre-existing generated `./perseus` subdirectory from the project.
RUN bonnie dev example $EXAMPLE_CATEGORY $EXAMPLE_NAME clean

# Prepare the project prior to deployment.
RUN bonnie dev example $EXAMPLE_CATEGORY $EXAMPLE_NAME prep

# Patch `clippy` inner attribute syntax error in `lib.rs` (if found).
RUN sed -i "s|\(#\)!\(\[allow(clippy::unused_unit)\]\)|\1\2|;" ./.perseus/src/lib.rs

# Single-threaded perseus CLI mode required for low memory environments.
# ENV PERSEUS_CLI_SEQUENTIAL=true

# Export variables required by `wasm-bindgen` and deploy the app from the project.
RUN export PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig \
  && bonnie dev example $EXAMPLE_CATEGORY $EXAMPLE_NAME deploy

# Work from the path containing the deployed app.
WORKDIR /app/perseus-${PERSEUS_BRANCH}/examples/${EXAMPLE_CATEGORY}/${EXAMPLE_NAME}

# Run `esbuild` against `bundle.js` to optimize it into minified format.
RUN esbuild ./pkg/dist/pkg/perseus_engine.js \
  --minify \
  --target=es6 \
  --outfile=./pkg/dist/pkg/perseus_engine.js \
  --allow-overwrite \
  && ls -lha ./pkg/dist/pkg

# Run `wasm-opt` against `bundle.wasm` to optimize it based on bytesize.
RUN wasm-opt \
  -Os ./pkg/dist/pkg/perseus_engine_bg.wasm \
  -o ./pkg/dist/pkg/perseus_engine_bg.wasm \
  && ls -lha ./pkg/dist/pkg

# Rename the dynamic path containing the deployed app to a static path.
RUN mv /app/perseus-${PERSEUS_BRANCH} /app/perseus-branch

# Prepare the final image where the app will be deployed.
FROM debian:stable-slim

# Work from a chosen install path for the deployed app.
WORKDIR /app

# Copy the deployed app into its chosen install path.
COPY --from=build /app/perseus-branch/examples/comprehensive/tiny/pkg /app/

# Bind the container to `localhost`.
ENV HOST=0.0.0.0

# Bind the container to the default port of `8080`.
ENV PORT=8080

# Configure the container to automatically serve the deployed app while running.
CMD ["./server"]
```

</details>
