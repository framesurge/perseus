# Docker Deployment

For situations where [serverful deployment](:deploying/serverful) is required, or in case there is a need to deploy one of the examples found on GitHub without prior setup of all necessary dependencies, below are `Dockerfile` examples meant to serve for different deployment scenarios. These steps can also serve as guidelines for production deployments.

Note that the following examples should be modified for your particular use-case rather than being used as-is. Also, these `Dockerfile`s are standalone because they use `curl` to download examples directly from the Perseus repository (of course, you'll probably want to use your own code in production).

Before proceeding with this section, you should be familiar with Docker's [multi-stage builds system](https://docs.docker.com/develop/develop-images/multistage-build) and Perseus' [code size optimizations](:deploying/size).

<details>
<summary>Production example using the size optimizations plugin</summary>

```dockerfile
# get the base image
FROM rust:1.57-slim AS build

# install build dependencies
RUN apt update \
  && apt -y install --no-install-recommends \
  apt-transport-https \
  build-essential \
  curl \
  libssl-dev \
  lsb-release \
  openssl \
  pkg-config

# vars
ENV PERSEUS_VERSION=0.3.3 \
  PERSEUS_SIZE_OPT_VERSION=0.1.7 \
  ESBUILD_VERSION=0.14.7 \
  BINARYEN_VERSION=104

# prepare root project dir
WORKDIR /app

# download the target for wasm
RUN rustup target add wasm32-unknown-unknown

# install wasm-pack
RUN cargo install wasm-pack

# retrieve the src dir
RUN curl -L \
  https://codeload.github.com/arctic-hen7/perseus-size-opt/tar.gz/v${PERSEUS_SIZE_OPT_VERSION} \
  | tar -xz --strip=2 perseus-size-opt-${PERSEUS_SIZE_OPT_VERSION}/examples/simple

# download, unpack, and verify install of binaryen
RUN curl -Lo binaryen-${BINARYEN_VERSION}.tar.gz \
  https://github.com/WebAssembly/binaryen/releases/download/version_${BINARYEN_VERSION}/binaryen-version_${BINARYEN_VERSION}-x86_64-linux.tar.gz \
  && tar -xzf binaryen-${BINARYEN_VERSION}.tar.gz \
  && ln -s $(pwd)/binaryen-version_${BINARYEN_VERSION}/bin/wasm-opt /usr/bin/wasm-opt \
  && wasm-opt --version

# go to src dir
WORKDIR /app/simple

# install perseus-cli
RUN cargo install perseus-cli --version $PERSEUS_VERSION

# specify deps in app config
RUN sed -i "\
  s|^\(perseus =\).*$|\1 \"${PERSEUS_VERSION}\"|g; \
  s|^\(perseus-size-opt =\).*$|\1 \"${PERSEUS_SIZE_OPT_VERSION}\"|g;" \
  ./Cargo.toml && cat ./Cargo.toml

# modify lib.rs
RUN sed -i "\
  s|\(\.plugin\)(\(perseus_size_opt,\) SizeOpts::default())$|\n\
  \1(\n\
    \2\n\
    SizeOpts {\n\
      wee_alloc: true,\n\
      lto: true,\n\
      opt_level: \"s\".to_string(),\n\
      codegen_units: 1,\n\
      enable_fluent_bundle_patch: false,\n\
    }\n\
  )|" ./src/lib.rs && cat ./src/lib.rs

# clean and prep app
RUN perseus clean && perseus prep

# run plugin(s) to adjust app
RUN perseus tinker \
  && cat .perseus/Cargo.toml \
  && cat ./src/lib.rs

# single-threaded perseus CLI mode required for low memory environments
#ENV PERSEUS_CLI_SEQUENTIAL=true

# export variables required by wasm-bindgen and deploy app
RUN export PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig \
  && perseus deploy

# go back to app dir
WORKDIR /app

# download, unpack, and verify install of esbuild
RUN curl -Lo esbuild-${ESBUILD_VERSION}.tar.gz \
  https://registry.npmjs.org/esbuild-linux-64/-/esbuild-linux-64-${ESBUILD_VERSION}.tgz \
  && tar -xzf esbuild-${ESBUILD_VERSION}.tar.gz \
  && ln -s $(pwd)/package/bin/esbuild /usr/bin/esbuild \
  && esbuild --version

# run esbuild against bundle.js
RUN esbuild ./simple/pkg/dist/pkg/perseus_engine.js \
  --minify \
  --target=es6 \
  --outfile=./simple/pkg/dist/pkg/perseus_engine.js \
  --allow-overwrite \
  && ls -lha ./simple/pkg/dist/pkg

# run wasm-opt against bundle.wasm
RUN wasm-opt \
  -Os ./simple/pkg/dist/pkg/perseus_engine_bg.wasm \
  -o ./simple/pkg/dist/pkg/perseus_engine_bg.wasm \
  && ls -lha ./simple/pkg/dist/pkg

# prepare deployment image
FROM debian:stable-slim

WORKDIR /app

COPY --from=build /app/simple/pkg /app/

ENV HOST=0.0.0.0

CMD ["./server"]
```

</details>

<details>
<summary>Production examples using `wee_alloc` manually</summary>

```dockerfile
# get the base image
FROM rust:1.57-slim AS build

# install build dependencies
RUN apt update \
  && apt -y install --no-install-recommends \
  apt-transport-https \
  build-essential \
  curl \
  libssl-dev \
  lsb-release \
  openssl \
  pkg-config

# vars
ENV PERSEUS_VERSION=0.3.3 \
  WEE_ALLOC_VERSION=0.4 \
  ESBUILD_VERSION=0.14.7 \
  BINARYEN_VERSION=104

# prepare root project dir
WORKDIR /app

# download the target for wasm
RUN rustup target add wasm32-unknown-unknown

# install wasm-pack
RUN cargo install wasm-pack

# retrieve the src dir
RUN curl -L \
  https://codeload.github.com/arctic-hen7/perseus/tar.gz/v${PERSEUS_VERSION} \
  | tar -xz --strip=3 perseus-${PERSEUS_VERSION}/examples/comprehensive/tiny

# download, unpack and verify install of binaryen
RUN curl -Lo binaryen-${BINARYEN_VERSION}.tar.gz \
  https://github.com/WebAssembly/binaryen/releases/download/version_${BINARYEN_VERSION}/binaryen-version_${BINARYEN_VERSION}-x86_64-linux.tar.gz \
  && tar -xzf binaryen-${BINARYEN_VERSION}.tar.gz \
  && ln -s $(pwd)/binaryen-version_${BINARYEN_VERSION}/bin/wasm-opt /usr/bin/wasm-opt \
  && wasm-opt --version

# go to src dir
WORKDIR /app/tiny

# install perseus-cli
RUN cargo install perseus-cli --version $PERSEUS_VERSION

# specify deps in app config
RUN sed -i "\
  s|^\(perseus =\).*$|\1 \"${PERSEUS_VERSION}\"|g; \
  s|^\(\[dependencies\]\)$|\1\n wee_alloc = \"${WEE_ALLOC_VERSION}\"|g;" \
  ./Cargo.toml && cat ./Cargo.toml

# modify and prepend lib.rs
RUN sed -i "1i \
  #[global_allocator]\n\
  static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;\n" \
  ./src/lib.rs && cat ./src/lib.rs

# clean, prep and eject app
RUN perseus clean && perseus prep && perseus eject

# adjust and append perseus config
RUN sed -i "s|^\(perseus =\).*$|\1 \"${PERSEUS_VERSION}\"|g" .perseus/Cargo.toml \
  && printf '%s\n' \
  "" "" \
  "[profile.release]" \
  "codegen-units = 1" \
  "opt-level = \"s\"" \
  "lto = true" >> .perseus/Cargo.toml \
  && cat .perseus/Cargo.toml

# single-threaded perseus CLI mode required for low memory environments
#ENV PERSEUS_CLI_SEQUENTIAL=true

# export variables required by wasm-bindgen and deploy app
RUN export PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig \
  && perseus deploy

# go back to app dir
WORKDIR /app

# download, unpack, and verify install of esbuild
RUN curl -Lo esbuild-${ESBUILD_VERSION}.tar.gz \
  https://registry.npmjs.org/esbuild-linux-64/-/esbuild-linux-64-${ESBUILD_VERSION}.tgz \
  && tar -xzf esbuild-${ESBUILD_VERSION}.tar.gz \
  && ln -s $(pwd)/package/bin/esbuild /usr/bin/esbuild \
  && esbuild --version

# run esbuild against bundle.js
RUN esbuild ./tiny/pkg/dist/pkg/perseus_engine.js \
  --minify \
  --target=es6 \
  --outfile=./tiny/pkg/dist/pkg/perseus_engine.js \
  --allow-overwrite \
  && ls -lha ./tiny/pkg/dist/pkg

# run wasm-opt against bundle.wasm
RUN wasm-opt \
  -Os ./tiny/pkg/dist/pkg/perseus_engine_bg.wasm \
  -o ./tiny/pkg/dist/pkg/perseus_engine_bg.wasm \
  && ls -lha ./tiny/pkg/dist/pkg

# prepare deployment image
FROM debian:stable-slim

WORKDIR /app

COPY --from=build /app/tiny/pkg /app/

ENV HOST=0.0.0.0

CMD ["./server"]
```

</details>

<details>
<summary>Test example for deploying a specific branch from the Perseus repository</summary>

```dockerfile
# get the base image
FROM rust:1.57-slim AS build

# install build dependencies
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

# vars
ENV PERSEUS_BRANCH=main

# prepare root project dir
WORKDIR /app

# download the target for wasm
RUN rustup target add wasm32-unknown-unknown

# install wasm-pack
RUN cargo install wasm-pack

# install bonnie
RUN cargo install bonnie

# retrieve the branch dir
RUN curl -L \
  https://codeload.github.com/arctic-hen7/perseus/tar.gz/${PERSEUS_BRANCH} \
  | tar -xz

# go to branch dir
WORKDIR /app/perseus-${PERSEUS_BRANCH}

# install perseus-cli from branch
RUN bonnie setup

# clean app
RUN bonnie dev example tiny clean

# single-threaded perseus CLI mode required for low memory environments
#ENV PERSEUS_CLI_SEQUENTIAL=true

# deploy app
RUN export PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig \
  && bonnie dev example tiny deploy

# move branch dir
RUN mv /app/perseus-${PERSEUS_BRANCH} /app/perseus-branch

# prepare deployment image
FROM debian:stable-slim

WORKDIR /app

COPY --from=build /app/perseus-branch/examples/tiny/pkg /app/

ENV HOST=0.0.0.0

CMD ["./server"]
```

</details>
