# Docker Deployment

For situations where [serverful deployment](:deploying/serverful) is required, or in case there is a need to deploy one of the examples found on GitHub without prior setup of all necessary dependencies, below are `Dockerfile` examples meant to serve for different deployment scenarios. These steps can also serve as guidelines for production deployments.

Note that the following examples should be modified for your particular use-case rather than being used as-is. Also, these `Dockerfile`s are standalone because they use `curl` to download examples directly from the Perseus repository (of course, you'll probably want to use your own code in production).

Before proceeding with this section, you should be familiar with Docker's [multi-stage builds system](https://docs.docker.com/develop/develop-images/multistage-build) and Perseus' [code size optimizations](:deploying/size).

<details>
<summary>Production example using the size optimizations plugin</summary>

<pre class="language-shell" tabindex="0">
<code class="language-shell">
# get the base image
FROM rust:1.57-slim AS build

# install build dependencies
RUN apt update \
  && apt install -y --no-install-recommends lsb-release apt-transport-https \
  build-essential curl wget

# vars
ENV PERSEUS_VERSION=0.3.0 \
  PERSEUS_SIZE_OPT_VERSION=0.1.7 \
  ESBUILD_VERSION=0.14.10 \
  BINARYEN_VERSION=104

# prepare root project dir
WORKDIR /app

# download the target for wasm
RUN rustup target add wasm32-unknown-unknown

# install wasm-pack
RUN cargo install wasm-pack

# retrieve the src dir
RUN curl https://codeload.github.com/arctic-hen7/perseus-size-opt/tar.gz/main | tar -xz --strip=2 perseus-size-opt-main/examples/simple

# go to src dir
WORKDIR /app/simple

# install perseus-cli
RUN cargo install perseus-cli --version $PERSEUS_VERSION

# clean and prep app
RUN perseus clean && perseus prep

# specify deps in app config
RUN sed -i s"/perseus = .*/perseus = \"${PERSEUS_VERSION}\"/" ./Cargo.toml \
  && sed -i s"/perseus-size-opt = .*/perseus-size-opt = \"${PERSEUS_SIZE_OPT_VERSION}\"/" ./Cargo.toml \
  && cat ./Cargo.toml

# modify lib.rs
RUN sed -i s'/SizeOpts::default()/SizeOpts { wee_alloc: true, lto: true, opt_level: "s".to_string(), codegen_units: 1, enable_fluent_bundle_patch: false, }/' ./src/lib.rs \
  && cat ./src/lib.rs

# run plugin(s) to adjust app
RUN perseus tinker \
  && cat .perseus/Cargo.toml \
  && cat ./src/lib.rs

# single-threaded perseus CLI mode required for low memory environments
#ENV PERSEUS_CLI_SEQUENTIAL=true

# deploy app
RUN perseus deploy

# go back to app dir
WORKDIR /app

# download and unpack esbuild
RUN curl -O https://registry.npmjs.org/esbuild-linux-64/-/esbuild-linux-64-${ESBUILD_VERSION}.tgz \
  && tar xf esbuild-linux-64-${ESBUILD_VERSION}.tgz \
  && ./package/bin/esbuild --version

# run esbuild against bundle.js
RUN ./package/bin/esbuild ./simple/pkg/dist/pkg/perseus_engine.js --minify --target=es6 --outfile=./simple/pkg/dist/pkg/perseus_engine.js --allow-overwrite \
  && ls -lha ./simple/pkg/dist/pkg

# download and unpack binaryen
RUN wget -nv https://github.com/WebAssembly/binaryen/releases/download/version_${BINARYEN_VERSION}/binaryen-version_${BINARYEN_VERSION}-x86_64-linux.tar.gz \
  && tar xf binaryen-version_${BINARYEN_VERSION}-x86_64-linux.tar.gz \
  && ./binaryen-version_${BINARYEN_VERSION}/bin/wasm-opt --version

# run wasm-opt against bundle.wasm
RUN ./binaryen-version_${BINARYEN_VERSION}/bin/wasm-opt -Os ./simple/pkg/dist/pkg/perseus_engine_bg.wasm -o ./simple/pkg/dist/pkg/perseus_engine_bg.wasm \
  && ls -lha ./simple/pkg/dist/pkg

# prepare deployment image
FROM debian:stable-slim

WORKDIR /app

COPY --from=build /app/simple/pkg /app/

ENV HOST=0.0.0.0

CMD ["./server"]
</code>
</pre>

</details>

<details>
<summary>Production examples using `wee_alloc` manually</summary>

<pre class="language-shell" tabindex="0">
<code class="language-shell">
# get the base image
FROM rust:1.57-slim AS build

# install build dependencies
RUN apt update \
  && apt install -y --no-install-recommends lsb-release apt-transport-https \
  build-essential curl wget

# vars
ENV PERSEUS_VERSION=0.3.0 \
  WEE_ALLOC_VERSION=0.4 \
  ESBUILD_VERSION=0.14.10 \
  BINARYEN_VERSION=104

# prepare root project dir
WORKDIR /app

# download the target for wasm
RUN rustup target add wasm32-unknown-unknown

# install wasm-pack
RUN cargo install wasm-pack

# retrieve the src dir
RUN curl https://codeload.github.com/arctic-hen7/perseus/tar.gz/v${PERSEUS_VERSION} | tar -xz --strip=2 perseus-${PERSEUS_VERSION}/examples/tiny

# go to src dir
WORKDIR /app/tiny

# install perseus-cli
RUN cargo install perseus-cli --version $PERSEUS_VERSION

# specify deps in app config
RUN sed -i s"/perseus = .*/perseus = \"${PERSEUS_VERSION}\"/" ./Cargo.toml \
  && sed -i "/\[dependencies\]/a wee_alloc = \"${WEE_ALLOC_VERSION}\"" ./Cargo.toml \
  && cat ./Cargo.toml

# modify and prepend lib.rs
RUN echo '#[global_allocator] \n\
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT; \n\
' | cat - ./src/lib.rs > ./src/lib.rs.tmp \
  && mv ./src/lib.rs.tmp ./src/lib.rs \
  && cat ./src/lib.rs

# clean, prep and eject app
RUN perseus clean && perseus prep && perseus eject

# adjust and append perseus config
RUN sed -i s"/perseus = .*/perseus = \"${PERSEUS_VERSION}\"/" .perseus/Cargo.toml \
  && echo ' \n\n\
[profile.release] \n\
codegen-units = 1 \n\
opt-level = "s" \n\
lto = true ' >> .perseus/Cargo.toml \
  && cat .perseus/Cargo.toml

# single-threaded perseus CLI mode required for low memory environments
#ENV PERSEUS_CLI_SEQUENTIAL=true

# deploy app
RUN perseus deploy

# go back to app dir
WORKDIR /app

# download and unpack esbuild
RUN curl -O https://registry.npmjs.org/esbuild-linux-64/-/esbuild-linux-64-${ESBUILD_VERSION}.tgz \
  && tar xf esbuild-linux-64-${ESBUILD_VERSION}.tgz \
  && ./package/bin/esbuild --version

# run esbuild against bundle.js
RUN ./package/bin/esbuild ./tiny/pkg/dist/pkg/perseus_engine.js --minify --target=es6 --outfile=./tiny/pkg/dist/pkg/perseus_engine.js --allow-overwrite \
  && ls -lha ./tiny/pkg/dist/pkg

# download and unpack binaryen
RUN wget -nv https://github.com/WebAssembly/binaryen/releases/download/version_${BINARYEN_VERSION}/binaryen-version_${BINARYEN_VERSION}-x86_64-linux.tar.gz \
  && tar xf binaryen-version_${BINARYEN_VERSION}-x86_64-linux.tar.gz \
  && ./binaryen-version_${BINARYEN_VERSION}/bin/wasm-opt --version

# run wasm-opt against bundle.wasm
RUN ./binaryen-version_${BINARYEN_VERSION}/bin/wasm-opt -Os ./tiny/pkg/dist/pkg/perseus_engine_bg.wasm -o ./tiny/pkg/dist/pkg/perseus_engine_bg.wasm \
  && ls -lha ./tiny/pkg/dist/pkg

# prepare deployment image
FROM debian:stable-slim

WORKDIR /app

COPY --from=build /app/tiny/pkg /app/

ENV HOST=0.0.0.0

CMD ["./server"]
</code>
</pre>

</details>

<details>
<summary>Test example for deploying a specific branch from the Perseus repository</summary>

<pre class="language-shell" tabindex="0">
<code class="language-shell">
# get the base image
FROM rust:1.57-slim AS build

# install build dependencies
RUN apt update \
  && apt install -y --no-install-recommends lsb-release apt-transport-https \
  build-essential curl

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
RUN curl https://codeload.github.com/arctic-hen7/perseus/tar.gz/${PERSEUS_BRANCH} | tar -xz

# go to branch dir
WORKDIR /app/perseus-${PERSEUS_BRANCH}

# install perseus-cli from branch
RUN bonnie setup

# clean app
RUN bonnie dev example tiny clean

# go to the branch dir
WORKDIR /app/perseus-${PERSEUS_BRANCH}

# single-threaded perseus CLI mode required for low memory environments
#ENV PERSEUS_CLI_SEQUENTIAL=true

# deploy app
RUN bonnie dev example tiny deploy

# move branch dir
RUN mv /app/perseus-${PERSEUS_BRANCH} /app/perseus-branch

# prepare deployment image
FROM debian:stable-slim

WORKDIR /app

COPY --from=build /app/perseus-branch/examples/tiny/pkg /app/

ENV HOST=0.0.0.0

CMD ["./server"]
</code>
</pre>

</details>
