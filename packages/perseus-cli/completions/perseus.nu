module completions {

  # The command-line interface for Perseus, a super-fast WebAssembly frontend development framework!
  export extern perseus [
    --cargo-engine-path: string # The path to `cargo` when used for engine builds
    --cargo-browser-path: string # The path to `cargo` when used for browser builds
    --wasm-bindgen-path: string # A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)
    --wasm-opt-path: string   # A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)
    --rustup-path: string     # The path to `rustup`
    --wasm-release-rustflags: string # The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)
    --cargo-engine-args: string # Any arguments to `cargo` when building for the engine-side
    --cargo-browser-args: string # Any arguments to `cargo` when building for the browser-side
    --wasm-bindgen-args: string # Any arguments to `wasm-bindgen`
    --wasm-opt-args: string   # Any arguments to `wasm-opt` (only run in release builds)
    --git-path: string        # The path to `git` (for downloading custom templates for `perseus new`)
    --reload-server-host: string # The host for the reload server (you should almost never change this)
    --reload-server-port: string # The port for the reload server (you should almost never change this)
    --sequential              # If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)
    --no-browser-reload       # Disable automatic browser reloading
    --wasm-bindgen-version: string # A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)
    --wasm-opt-version: string # A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)
    --no-system-tools-cache   # Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)
    --verbose                 # Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging
    --disable-bundle-compression # Disable Brotli compression of JS and Wasm bundles (may degrade performance)
    --help(-h)                # Print help
    --version(-V)             # Print version
  ]

  # Builds your app
  export extern "perseus build" [
    --release                 # Build for production
    --watch(-w)               # Watch the files in your working directory for changes (excluding `target/` and `dist/`)
    --custom-watch: string    # Marks a specific file/directory to be watched (directories will be recursively watched)
    --cargo-engine-path: string # The path to `cargo` when used for engine builds
    --cargo-browser-path: string # The path to `cargo` when used for browser builds
    --wasm-bindgen-path: string # A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)
    --wasm-opt-path: string   # A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)
    --rustup-path: string     # The path to `rustup`
    --wasm-release-rustflags: string # The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)
    --cargo-engine-args: string # Any arguments to `cargo` when building for the engine-side
    --cargo-browser-args: string # Any arguments to `cargo` when building for the browser-side
    --wasm-bindgen-args: string # Any arguments to `wasm-bindgen`
    --wasm-opt-args: string   # Any arguments to `wasm-opt` (only run in release builds)
    --git-path: string        # The path to `git` (for downloading custom templates for `perseus new`)
    --reload-server-host: string # The host for the reload server (you should almost never change this)
    --reload-server-port: string # The port for the reload server (you should almost never change this)
    --sequential              # If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)
    --no-browser-reload       # Disable automatic browser reloading
    --wasm-bindgen-version: string # A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)
    --wasm-opt-version: string # A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)
    --no-system-tools-cache   # Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)
    --verbose                 # Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging
    --disable-bundle-compression # Disable Brotli compression of JS and Wasm bundles (may degrade performance)
    --help(-h)                # Print help
  ]

  # Exports an error page for the given HTTP status code
  export extern "perseus export-error-page" [
    --code(-c): string
    --output(-o): string
    --cargo-engine-path: string # The path to `cargo` when used for engine builds
    --cargo-browser-path: string # The path to `cargo` when used for browser builds
    --wasm-bindgen-path: string # A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)
    --wasm-opt-path: string   # A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)
    --rustup-path: string     # The path to `rustup`
    --wasm-release-rustflags: string # The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)
    --cargo-engine-args: string # Any arguments to `cargo` when building for the engine-side
    --cargo-browser-args: string # Any arguments to `cargo` when building for the browser-side
    --wasm-bindgen-args: string # Any arguments to `wasm-bindgen`
    --wasm-opt-args: string   # Any arguments to `wasm-opt` (only run in release builds)
    --git-path: string        # The path to `git` (for downloading custom templates for `perseus new`)
    --reload-server-host: string # The host for the reload server (you should almost never change this)
    --reload-server-port: string # The port for the reload server (you should almost never change this)
    --sequential              # If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)
    --no-browser-reload       # Disable automatic browser reloading
    --wasm-bindgen-version: string # A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)
    --wasm-opt-version: string # A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)
    --no-system-tools-cache   # Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)
    --verbose                 # Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging
    --disable-bundle-compression # Disable Brotli compression of JS and Wasm bundles (may degrade performance)
    --help(-h)                # Print help
  ]

  # Exports your app to purely static files
  export extern "perseus export" [
    --release                 # Export for production
    --serve(-s)               # Serve the generated static files locally
    --host: string            # Where to host your exported app
    --port: string            # The port to host your exported app on
    --watch(-w)               # Watch the files in your working directory for changes (excluding `target/` and `dist/`)
    --custom-watch: string    # Marks a specific file/directory to be watched (directories will be recursively watched)
    --cargo-engine-path: string # The path to `cargo` when used for engine builds
    --cargo-browser-path: string # The path to `cargo` when used for browser builds
    --wasm-bindgen-path: string # A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)
    --wasm-opt-path: string   # A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)
    --rustup-path: string     # The path to `rustup`
    --wasm-release-rustflags: string # The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)
    --cargo-engine-args: string # Any arguments to `cargo` when building for the engine-side
    --cargo-browser-args: string # Any arguments to `cargo` when building for the browser-side
    --wasm-bindgen-args: string # Any arguments to `wasm-bindgen`
    --wasm-opt-args: string   # Any arguments to `wasm-opt` (only run in release builds)
    --git-path: string        # The path to `git` (for downloading custom templates for `perseus new`)
    --reload-server-host: string # The host for the reload server (you should almost never change this)
    --reload-server-port: string # The port for the reload server (you should almost never change this)
    --sequential              # If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)
    --no-browser-reload       # Disable automatic browser reloading
    --wasm-bindgen-version: string # A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)
    --wasm-opt-version: string # A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)
    --no-system-tools-cache   # Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)
    --verbose                 # Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging
    --disable-bundle-compression # Disable Brotli compression of JS and Wasm bundles (may degrade performance)
    --help(-h)                # Print help
  ]

  # Serves your app
  export extern "perseus serve" [
    --no-run                  # Don't run the final binary, but print its location instead as the last line of output
    --no-build                # Only build the server, and use the results of a previous `perseus build`
    --release                 # Build and serve for production
    --standalone              # Make the final binary standalone (this is used in `perseus deploy` only, don't manually invoke it unless you have a good reason!)
    --watch(-w)               # Watch the files in your working directory for changes (excluding `target/` and `dist/`)
    --custom-watch: string    # Marks a specific file/directory to be watched (directories will be recursively watched)
    --host: string            # Where to host your exported app
    --port: string            # The port to host your exported app on
    --cargo-engine-path: string # The path to `cargo` when used for engine builds
    --cargo-browser-path: string # The path to `cargo` when used for browser builds
    --wasm-bindgen-path: string # A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)
    --wasm-opt-path: string   # A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)
    --rustup-path: string     # The path to `rustup`
    --wasm-release-rustflags: string # The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)
    --cargo-engine-args: string # Any arguments to `cargo` when building for the engine-side
    --cargo-browser-args: string # Any arguments to `cargo` when building for the browser-side
    --wasm-bindgen-args: string # Any arguments to `wasm-bindgen`
    --wasm-opt-args: string   # Any arguments to `wasm-opt` (only run in release builds)
    --git-path: string        # The path to `git` (for downloading custom templates for `perseus new`)
    --reload-server-host: string # The host for the reload server (you should almost never change this)
    --reload-server-port: string # The port for the reload server (you should almost never change this)
    --sequential              # If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)
    --no-browser-reload       # Disable automatic browser reloading
    --wasm-bindgen-version: string # A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)
    --wasm-opt-version: string # A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)
    --no-system-tools-cache   # Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)
    --verbose                 # Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging
    --disable-bundle-compression # Disable Brotli compression of JS and Wasm bundles (may degrade performance)
    --help(-h)                # Print help
  ]

  # Serves your app as `perseus serve` does, but puts it in testing mode
  export extern "perseus test" [
    --no-build                # Only build the testing server, and use the results of a previous `perseus build`
    --show-browser            # Show the browser window when testing (by default, the browser is used in 'headless' mode); this can be useful for debugging failing tests in some cases
    --watch(-w)               # Watch the files in your working directory for changes (excluding `target/` and `dist/`)
    --custom-watch: string    # Marks a specific file/directory to be watched (directories will be recursively watched)
    --host: string            # Where to host your exported app
    --port: string            # The port to host your exported app on
    --cargo-engine-path: string # The path to `cargo` when used for engine builds
    --cargo-browser-path: string # The path to `cargo` when used for browser builds
    --wasm-bindgen-path: string # A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)
    --wasm-opt-path: string   # A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)
    --rustup-path: string     # The path to `rustup`
    --wasm-release-rustflags: string # The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)
    --cargo-engine-args: string # Any arguments to `cargo` when building for the engine-side
    --cargo-browser-args: string # Any arguments to `cargo` when building for the browser-side
    --wasm-bindgen-args: string # Any arguments to `wasm-bindgen`
    --wasm-opt-args: string   # Any arguments to `wasm-opt` (only run in release builds)
    --git-path: string        # The path to `git` (for downloading custom templates for `perseus new`)
    --reload-server-host: string # The host for the reload server (you should almost never change this)
    --reload-server-port: string # The port for the reload server (you should almost never change this)
    --sequential              # If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)
    --no-browser-reload       # Disable automatic browser reloading
    --wasm-bindgen-version: string # A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)
    --wasm-opt-version: string # A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)
    --no-system-tools-cache   # Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)
    --verbose                 # Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging
    --disable-bundle-compression # Disable Brotli compression of JS and Wasm bundles (may degrade performance)
    --help(-h)                # Print help
  ]

  # Removes build artifacts in the `dist/` directory
  export extern "perseus clean" [
    --cargo-engine-path: string # The path to `cargo` when used for engine builds
    --cargo-browser-path: string # The path to `cargo` when used for browser builds
    --wasm-bindgen-path: string # A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)
    --wasm-opt-path: string   # A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)
    --rustup-path: string     # The path to `rustup`
    --wasm-release-rustflags: string # The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)
    --cargo-engine-args: string # Any arguments to `cargo` when building for the engine-side
    --cargo-browser-args: string # Any arguments to `cargo` when building for the browser-side
    --wasm-bindgen-args: string # Any arguments to `wasm-bindgen`
    --wasm-opt-args: string   # Any arguments to `wasm-opt` (only run in release builds)
    --git-path: string        # The path to `git` (for downloading custom templates for `perseus new`)
    --reload-server-host: string # The host for the reload server (you should almost never change this)
    --reload-server-port: string # The port for the reload server (you should almost never change this)
    --sequential              # If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)
    --no-browser-reload       # Disable automatic browser reloading
    --wasm-bindgen-version: string # A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)
    --wasm-opt-version: string # A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)
    --no-system-tools-cache   # Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)
    --verbose                 # Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging
    --disable-bundle-compression # Disable Brotli compression of JS and Wasm bundles (may degrade performance)
    --help(-h)                # Print help
  ]

  # Packages your app for deployment
  export extern "perseus deploy" [
    --output(-o): string      # Change the output from `pkg/` to somewhere else
    --export-static(-e)       # Export your app to purely static files (see `export`)
    --no-minify-js            # Don't minify JavaScript (this will decrease performance)
    --cargo-engine-path: string # The path to `cargo` when used for engine builds
    --cargo-browser-path: string # The path to `cargo` when used for browser builds
    --wasm-bindgen-path: string # A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)
    --wasm-opt-path: string   # A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)
    --rustup-path: string     # The path to `rustup`
    --wasm-release-rustflags: string # The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)
    --cargo-engine-args: string # Any arguments to `cargo` when building for the engine-side
    --cargo-browser-args: string # Any arguments to `cargo` when building for the browser-side
    --wasm-bindgen-args: string # Any arguments to `wasm-bindgen`
    --wasm-opt-args: string   # Any arguments to `wasm-opt` (only run in release builds)
    --git-path: string        # The path to `git` (for downloading custom templates for `perseus new`)
    --reload-server-host: string # The host for the reload server (you should almost never change this)
    --reload-server-port: string # The port for the reload server (you should almost never change this)
    --sequential              # If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)
    --no-browser-reload       # Disable automatic browser reloading
    --wasm-bindgen-version: string # A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)
    --wasm-opt-version: string # A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)
    --no-system-tools-cache   # Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)
    --verbose                 # Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging
    --disable-bundle-compression # Disable Brotli compression of JS and Wasm bundles (may degrade performance)
    --help(-h)                # Print help
  ]

  # Runs the `tinker` action of plugins, which lets them modify the Perseus engine
  export extern "perseus tinker" [
    --no-clean                # Don't remove and recreate the `dist/` directory
    --cargo-engine-path: string # The path to `cargo` when used for engine builds
    --cargo-browser-path: string # The path to `cargo` when used for browser builds
    --wasm-bindgen-path: string # A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)
    --wasm-opt-path: string   # A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)
    --rustup-path: string     # The path to `rustup`
    --wasm-release-rustflags: string # The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)
    --cargo-engine-args: string # Any arguments to `cargo` when building for the engine-side
    --cargo-browser-args: string # Any arguments to `cargo` when building for the browser-side
    --wasm-bindgen-args: string # Any arguments to `wasm-bindgen`
    --wasm-opt-args: string   # Any arguments to `wasm-opt` (only run in release builds)
    --git-path: string        # The path to `git` (for downloading custom templates for `perseus new`)
    --reload-server-host: string # The host for the reload server (you should almost never change this)
    --reload-server-port: string # The port for the reload server (you should almost never change this)
    --sequential              # If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)
    --no-browser-reload       # Disable automatic browser reloading
    --wasm-bindgen-version: string # A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)
    --wasm-opt-version: string # A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)
    --no-system-tools-cache   # Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)
    --verbose                 # Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging
    --disable-bundle-compression # Disable Brotli compression of JS and Wasm bundles (may degrade performance)
    --help(-h)                # Print help
  ]

  # Runs one of the underlying commands that builds your app, allowing you to see more detailed logs
  export extern "perseus snoop" [
    --cargo-engine-path: string # The path to `cargo` when used for engine builds
    --cargo-browser-path: string # The path to `cargo` when used for browser builds
    --wasm-bindgen-path: string # A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)
    --wasm-opt-path: string   # A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)
    --rustup-path: string     # The path to `rustup`
    --wasm-release-rustflags: string # The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)
    --cargo-engine-args: string # Any arguments to `cargo` when building for the engine-side
    --cargo-browser-args: string # Any arguments to `cargo` when building for the browser-side
    --wasm-bindgen-args: string # Any arguments to `wasm-bindgen`
    --wasm-opt-args: string   # Any arguments to `wasm-opt` (only run in release builds)
    --git-path: string        # The path to `git` (for downloading custom templates for `perseus new`)
    --reload-server-host: string # The host for the reload server (you should almost never change this)
    --reload-server-port: string # The port for the reload server (you should almost never change this)
    --sequential              # If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)
    --no-browser-reload       # Disable automatic browser reloading
    --wasm-bindgen-version: string # A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)
    --wasm-opt-version: string # A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)
    --no-system-tools-cache   # Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)
    --verbose                 # Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging
    --disable-bundle-compression # Disable Brotli compression of JS and Wasm bundles (may degrade performance)
    --help(-h)                # Print help
  ]

  # Snoops on the static generation process (this will let you see `dbg!` calls and the like)
  export extern "perseus snoop build" [
    --watch(-w)               # Watch the files in your working directory for changes (excluding `target/` and `dist/`)
    --custom-watch: string    # Marks a specific file/directory to be watched (directories will be recursively watched)
    --cargo-engine-path: string # The path to `cargo` when used for engine builds
    --cargo-browser-path: string # The path to `cargo` when used for browser builds
    --wasm-bindgen-path: string # A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)
    --wasm-opt-path: string   # A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)
    --rustup-path: string     # The path to `rustup`
    --wasm-release-rustflags: string # The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)
    --cargo-engine-args: string # Any arguments to `cargo` when building for the engine-side
    --cargo-browser-args: string # Any arguments to `cargo` when building for the browser-side
    --wasm-bindgen-args: string # Any arguments to `wasm-bindgen`
    --wasm-opt-args: string   # Any arguments to `wasm-opt` (only run in release builds)
    --git-path: string        # The path to `git` (for downloading custom templates for `perseus new`)
    --reload-server-host: string # The host for the reload server (you should almost never change this)
    --reload-server-port: string # The port for the reload server (you should almost never change this)
    --sequential              # If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)
    --no-browser-reload       # Disable automatic browser reloading
    --wasm-bindgen-version: string # A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)
    --wasm-opt-version: string # A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)
    --no-system-tools-cache   # Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)
    --verbose                 # Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging
    --disable-bundle-compression # Disable Brotli compression of JS and Wasm bundles (may degrade performance)
    --help(-h)                # Print help
  ]

  # Snoops on the Wasm building process (mostly for debugging errors)
  export extern "perseus snoop wasm-build" [
    --watch(-w)               # Watch the files in your working directory for changes (excluding `target/` and `dist/`)
    --custom-watch: string    # Marks a specific file/directory to be watched (directories will be recursively watched)
    --cargo-engine-path: string # The path to `cargo` when used for engine builds
    --cargo-browser-path: string # The path to `cargo` when used for browser builds
    --wasm-bindgen-path: string # A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)
    --wasm-opt-path: string   # A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)
    --rustup-path: string     # The path to `rustup`
    --wasm-release-rustflags: string # The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)
    --cargo-engine-args: string # Any arguments to `cargo` when building for the engine-side
    --cargo-browser-args: string # Any arguments to `cargo` when building for the browser-side
    --wasm-bindgen-args: string # Any arguments to `wasm-bindgen`
    --wasm-opt-args: string   # Any arguments to `wasm-opt` (only run in release builds)
    --git-path: string        # The path to `git` (for downloading custom templates for `perseus new`)
    --reload-server-host: string # The host for the reload server (you should almost never change this)
    --reload-server-port: string # The port for the reload server (you should almost never change this)
    --sequential              # If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)
    --no-browser-reload       # Disable automatic browser reloading
    --wasm-bindgen-version: string # A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)
    --wasm-opt-version: string # A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)
    --no-system-tools-cache   # Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)
    --verbose                 # Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging
    --disable-bundle-compression # Disable Brotli compression of JS and Wasm bundles (may degrade performance)
    --help(-h)                # Print help
  ]

  # Snoops on the server process (run `perseus build` before this)
  export extern "perseus snoop serve" [
    --host: string            # Where to host your exported app
    --port: string            # The port to host your exported app on
    --watch(-w)               # Watch the files in your working directory for changes (excluding `target/` and `dist/`)
    --custom-watch: string    # Marks a specific file/directory to be watched (directories will be recursively watched)
    --cargo-engine-path: string # The path to `cargo` when used for engine builds
    --cargo-browser-path: string # The path to `cargo` when used for browser builds
    --wasm-bindgen-path: string # A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)
    --wasm-opt-path: string   # A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)
    --rustup-path: string     # The path to `rustup`
    --wasm-release-rustflags: string # The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)
    --cargo-engine-args: string # Any arguments to `cargo` when building for the engine-side
    --cargo-browser-args: string # Any arguments to `cargo` when building for the browser-side
    --wasm-bindgen-args: string # Any arguments to `wasm-bindgen`
    --wasm-opt-args: string   # Any arguments to `wasm-opt` (only run in release builds)
    --git-path: string        # The path to `git` (for downloading custom templates for `perseus new`)
    --reload-server-host: string # The host for the reload server (you should almost never change this)
    --reload-server-port: string # The port for the reload server (you should almost never change this)
    --sequential              # If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)
    --no-browser-reload       # Disable automatic browser reloading
    --wasm-bindgen-version: string # A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)
    --wasm-opt-version: string # A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)
    --no-system-tools-cache   # Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)
    --verbose                 # Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging
    --disable-bundle-compression # Disable Brotli compression of JS and Wasm bundles (may degrade performance)
    --help(-h)                # Print help
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "perseus snoop help" [
  ]

  # Snoops on the static generation process (this will let you see `dbg!` calls and the like)
  export extern "perseus snoop help build" [
  ]

  # Snoops on the Wasm building process (mostly for debugging errors)
  export extern "perseus snoop help wasm-build" [
  ]

  # Snoops on the server process (run `perseus build` before this)
  export extern "perseus snoop help serve" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "perseus snoop help help" [
  ]

  # Creates a new Perseus project in a directory of the given name, which will be created in the current path
  export extern "perseus new" [
    name: string              # The name of the new project, which will also be used for the directory
    --template(-t): string    # An optional custom URL to a Git repository to be used as a custom template (note that custom templates will not respect your project's name). This can be followed with `@branch` to fetch from `branch` rather than the default
    --dir: string             # The path to a custom directory to create (if this is not provided, the project name will be used by default)
    --cargo-engine-path: string # The path to `cargo` when used for engine builds
    --cargo-browser-path: string # The path to `cargo` when used for browser builds
    --wasm-bindgen-path: string # A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)
    --wasm-opt-path: string   # A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)
    --rustup-path: string     # The path to `rustup`
    --wasm-release-rustflags: string # The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)
    --cargo-engine-args: string # Any arguments to `cargo` when building for the engine-side
    --cargo-browser-args: string # Any arguments to `cargo` when building for the browser-side
    --wasm-bindgen-args: string # Any arguments to `wasm-bindgen`
    --wasm-opt-args: string   # Any arguments to `wasm-opt` (only run in release builds)
    --git-path: string        # The path to `git` (for downloading custom templates for `perseus new`)
    --reload-server-host: string # The host for the reload server (you should almost never change this)
    --reload-server-port: string # The port for the reload server (you should almost never change this)
    --sequential              # If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)
    --no-browser-reload       # Disable automatic browser reloading
    --wasm-bindgen-version: string # A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)
    --wasm-opt-version: string # A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)
    --no-system-tools-cache   # Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)
    --verbose                 # Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging
    --disable-bundle-compression # Disable Brotli compression of JS and Wasm bundles (may degrade performance)
    --help(-h)                # Print help
  ]

  # Initializes a new Perseus project in the current directory
  export extern "perseus init" [
    name: string              # The name of the new project
    --cargo-engine-path: string # The path to `cargo` when used for engine builds
    --cargo-browser-path: string # The path to `cargo` when used for browser builds
    --wasm-bindgen-path: string # A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)
    --wasm-opt-path: string   # A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)
    --rustup-path: string     # The path to `rustup`
    --wasm-release-rustflags: string # The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)
    --cargo-engine-args: string # Any arguments to `cargo` when building for the engine-side
    --cargo-browser-args: string # Any arguments to `cargo` when building for the browser-side
    --wasm-bindgen-args: string # Any arguments to `wasm-bindgen`
    --wasm-opt-args: string   # Any arguments to `wasm-opt` (only run in release builds)
    --git-path: string        # The path to `git` (for downloading custom templates for `perseus new`)
    --reload-server-host: string # The host for the reload server (you should almost never change this)
    --reload-server-port: string # The port for the reload server (you should almost never change this)
    --sequential              # If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)
    --no-browser-reload       # Disable automatic browser reloading
    --wasm-bindgen-version: string # A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)
    --wasm-opt-version: string # A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)
    --no-system-tools-cache   # Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)
    --verbose                 # Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging
    --disable-bundle-compression # Disable Brotli compression of JS and Wasm bundles (may degrade performance)
    --help(-h)                # Print help
  ]

  # Checks if your app builds properly for both the engine-side and the browser-side
  export extern "perseus check" [
    --watch(-w)               # Watch the files in your working directory for changes (excluding `target/` and `dist/`)
    --custom-watch: string    # Marks a specific file/directory to be watched (directories will be recursively watched)
    --generate(-g)            # Make sure the app's page generation works properly (this will take much longer, but almost guarantees that your app will actually build); use this to catch errors in build state and the like
    --cargo-engine-path: string # The path to `cargo` when used for engine builds
    --cargo-browser-path: string # The path to `cargo` when used for browser builds
    --wasm-bindgen-path: string # A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)
    --wasm-opt-path: string   # A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)
    --rustup-path: string     # The path to `rustup`
    --wasm-release-rustflags: string # The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)
    --cargo-engine-args: string # Any arguments to `cargo` when building for the engine-side
    --cargo-browser-args: string # Any arguments to `cargo` when building for the browser-side
    --wasm-bindgen-args: string # Any arguments to `wasm-bindgen`
    --wasm-opt-args: string   # Any arguments to `wasm-opt` (only run in release builds)
    --git-path: string        # The path to `git` (for downloading custom templates for `perseus new`)
    --reload-server-host: string # The host for the reload server (you should almost never change this)
    --reload-server-port: string # The port for the reload server (you should almost never change this)
    --sequential              # If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)
    --no-browser-reload       # Disable automatic browser reloading
    --wasm-bindgen-version: string # A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)
    --wasm-opt-version: string # A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)
    --no-system-tools-cache   # Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)
    --verbose                 # Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging
    --disable-bundle-compression # Disable Brotli compression of JS and Wasm bundles (may degrade performance)
    --help(-h)                # Print help
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "perseus help" [
  ]

  # Builds your app
  export extern "perseus help build" [
  ]

  # Exports an error page for the given HTTP status code
  export extern "perseus help export-error-page" [
  ]

  # Exports your app to purely static files
  export extern "perseus help export" [
  ]

  # Serves your app
  export extern "perseus help serve" [
  ]

  # Serves your app as `perseus serve` does, but puts it in testing mode
  export extern "perseus help test" [
  ]

  # Removes build artifacts in the `dist/` directory
  export extern "perseus help clean" [
  ]

  # Packages your app for deployment
  export extern "perseus help deploy" [
  ]

  # Runs the `tinker` action of plugins, which lets them modify the Perseus engine
  export extern "perseus help tinker" [
  ]

  # Runs one of the underlying commands that builds your app, allowing you to see more detailed logs
  export extern "perseus help snoop" [
  ]

  # Snoops on the static generation process (this will let you see `dbg!` calls and the like)
  export extern "perseus help snoop build" [
  ]

  # Snoops on the Wasm building process (mostly for debugging errors)
  export extern "perseus help snoop wasm-build" [
  ]

  # Snoops on the server process (run `perseus build` before this)
  export extern "perseus help snoop serve" [
  ]

  # Creates a new Perseus project in a directory of the given name, which will be created in the current path
  export extern "perseus help new" [
  ]

  # Initializes a new Perseus project in the current directory
  export extern "perseus help init" [
  ]

  # Checks if your app builds properly for both the engine-side and the browser-side
  export extern "perseus help check" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "perseus help help" [
  ]

}

export use completions *
