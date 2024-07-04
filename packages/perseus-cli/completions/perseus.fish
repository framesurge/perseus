complete -c perseus -n "__fish_use_subcommand" -l cargo-engine-path -d 'The path to `cargo` when used for engine builds' -r
complete -c perseus -n "__fish_use_subcommand" -l cargo-browser-path -d 'The path to `cargo` when used for browser builds' -r
complete -c perseus -n "__fish_use_subcommand" -l wasm-bindgen-path -d 'A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)' -r
complete -c perseus -n "__fish_use_subcommand" -l wasm-opt-path -d 'A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)' -r
complete -c perseus -n "__fish_use_subcommand" -l rustup-path -d 'The path to `rustup`' -r
complete -c perseus -n "__fish_use_subcommand" -l wasm-release-rustflags -d 'The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)' -r
complete -c perseus -n "__fish_use_subcommand" -l cargo-engine-args -d 'Any arguments to `cargo` when building for the engine-side' -r
complete -c perseus -n "__fish_use_subcommand" -l cargo-browser-args -d 'Any arguments to `cargo` when building for the browser-side' -r
complete -c perseus -n "__fish_use_subcommand" -l wasm-bindgen-args -d 'Any arguments to `wasm-bindgen`' -r
complete -c perseus -n "__fish_use_subcommand" -l wasm-opt-args -d 'Any arguments to `wasm-opt` (only run in release builds)' -r
complete -c perseus -n "__fish_use_subcommand" -l git-path -d 'The path to `git` (for downloading custom templates for `perseus new`)' -r
complete -c perseus -n "__fish_use_subcommand" -l reload-server-host -d 'The host for the reload server (you should almost never change this)' -r
complete -c perseus -n "__fish_use_subcommand" -l reload-server-port -d 'The port for the reload server (you should almost never change this)' -r
complete -c perseus -n "__fish_use_subcommand" -l wasm-bindgen-version -d 'A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)' -r
complete -c perseus -n "__fish_use_subcommand" -l wasm-opt-version -d 'A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)' -r
complete -c perseus -n "__fish_use_subcommand" -l sequential -d 'If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)'
complete -c perseus -n "__fish_use_subcommand" -l no-browser-reload -d 'Disable automatic browser reloading'
complete -c perseus -n "__fish_use_subcommand" -l no-system-tools-cache -d 'Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)'
complete -c perseus -n "__fish_use_subcommand" -l verbose -d 'Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging'
complete -c perseus -n "__fish_use_subcommand" -l disable-bundle-compression -d 'Disable Brotli compression of JS and Wasm bundles (may degrade performance)'
complete -c perseus -n "__fish_use_subcommand" -s h -l help -d 'Print help'
complete -c perseus -n "__fish_use_subcommand" -s V -l version -d 'Print version'
complete -c perseus -n "__fish_use_subcommand" -f -a "build" -d 'Builds your app'
complete -c perseus -n "__fish_use_subcommand" -f -a "export-error-page" -d 'Exports an error page for the given HTTP status code'
complete -c perseus -n "__fish_use_subcommand" -f -a "export" -d 'Exports your app to purely static files'
complete -c perseus -n "__fish_use_subcommand" -f -a "serve" -d 'Serves your app'
complete -c perseus -n "__fish_use_subcommand" -f -a "test" -d 'Serves your app as `perseus serve` does, but puts it in testing mode'
complete -c perseus -n "__fish_use_subcommand" -f -a "clean" -d 'Removes build artifacts in the `dist/` directory'
complete -c perseus -n "__fish_use_subcommand" -f -a "deploy" -d 'Packages your app for deployment'
complete -c perseus -n "__fish_use_subcommand" -f -a "tinker" -d 'Runs the `tinker` action of plugins, which lets them modify the Perseus engine'
complete -c perseus -n "__fish_use_subcommand" -f -a "snoop" -d 'Runs one of the underlying commands that builds your app, allowing you to see more detailed logs'
complete -c perseus -n "__fish_use_subcommand" -f -a "new" -d 'Creates a new Perseus project in a directory of the given name, which will be created in the current path'
complete -c perseus -n "__fish_use_subcommand" -f -a "init" -d 'Initializes a new Perseus project in the current directory'
complete -c perseus -n "__fish_use_subcommand" -f -a "check" -d 'Checks if your app builds properly for both the engine-side and the browser-side'
complete -c perseus -n "__fish_use_subcommand" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c perseus -n "__fish_seen_subcommand_from build" -l custom-watch -d 'Marks a specific file/directory to be watched (directories will be recursively watched)' -r
complete -c perseus -n "__fish_seen_subcommand_from build" -l cargo-engine-path -d 'The path to `cargo` when used for engine builds' -r
complete -c perseus -n "__fish_seen_subcommand_from build" -l cargo-browser-path -d 'The path to `cargo` when used for browser builds' -r
complete -c perseus -n "__fish_seen_subcommand_from build" -l wasm-bindgen-path -d 'A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)' -r
complete -c perseus -n "__fish_seen_subcommand_from build" -l wasm-opt-path -d 'A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)' -r
complete -c perseus -n "__fish_seen_subcommand_from build" -l rustup-path -d 'The path to `rustup`' -r
complete -c perseus -n "__fish_seen_subcommand_from build" -l wasm-release-rustflags -d 'The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)' -r
complete -c perseus -n "__fish_seen_subcommand_from build" -l cargo-engine-args -d 'Any arguments to `cargo` when building for the engine-side' -r
complete -c perseus -n "__fish_seen_subcommand_from build" -l cargo-browser-args -d 'Any arguments to `cargo` when building for the browser-side' -r
complete -c perseus -n "__fish_seen_subcommand_from build" -l wasm-bindgen-args -d 'Any arguments to `wasm-bindgen`' -r
complete -c perseus -n "__fish_seen_subcommand_from build" -l wasm-opt-args -d 'Any arguments to `wasm-opt` (only run in release builds)' -r
complete -c perseus -n "__fish_seen_subcommand_from build" -l git-path -d 'The path to `git` (for downloading custom templates for `perseus new`)' -r
complete -c perseus -n "__fish_seen_subcommand_from build" -l reload-server-host -d 'The host for the reload server (you should almost never change this)' -r
complete -c perseus -n "__fish_seen_subcommand_from build" -l reload-server-port -d 'The port for the reload server (you should almost never change this)' -r
complete -c perseus -n "__fish_seen_subcommand_from build" -l wasm-bindgen-version -d 'A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)' -r
complete -c perseus -n "__fish_seen_subcommand_from build" -l wasm-opt-version -d 'A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)' -r
complete -c perseus -n "__fish_seen_subcommand_from build" -l release -d 'Build for production'
complete -c perseus -n "__fish_seen_subcommand_from build" -s w -l watch -d 'Watch the files in your working directory for changes (excluding `target/` and `dist/`)'
complete -c perseus -n "__fish_seen_subcommand_from build" -l sequential -d 'If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)'
complete -c perseus -n "__fish_seen_subcommand_from build" -l no-browser-reload -d 'Disable automatic browser reloading'
complete -c perseus -n "__fish_seen_subcommand_from build" -l no-system-tools-cache -d 'Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)'
complete -c perseus -n "__fish_seen_subcommand_from build" -l verbose -d 'Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging'
complete -c perseus -n "__fish_seen_subcommand_from build" -l disable-bundle-compression -d 'Disable Brotli compression of JS and Wasm bundles (may degrade performance)'
complete -c perseus -n "__fish_seen_subcommand_from build" -s h -l help -d 'Print help'
complete -c perseus -n "__fish_seen_subcommand_from export-error-page" -s c -l code -r
complete -c perseus -n "__fish_seen_subcommand_from export-error-page" -s o -l output -r
complete -c perseus -n "__fish_seen_subcommand_from export-error-page" -l cargo-engine-path -d 'The path to `cargo` when used for engine builds' -r
complete -c perseus -n "__fish_seen_subcommand_from export-error-page" -l cargo-browser-path -d 'The path to `cargo` when used for browser builds' -r
complete -c perseus -n "__fish_seen_subcommand_from export-error-page" -l wasm-bindgen-path -d 'A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)' -r
complete -c perseus -n "__fish_seen_subcommand_from export-error-page" -l wasm-opt-path -d 'A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)' -r
complete -c perseus -n "__fish_seen_subcommand_from export-error-page" -l rustup-path -d 'The path to `rustup`' -r
complete -c perseus -n "__fish_seen_subcommand_from export-error-page" -l wasm-release-rustflags -d 'The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)' -r
complete -c perseus -n "__fish_seen_subcommand_from export-error-page" -l cargo-engine-args -d 'Any arguments to `cargo` when building for the engine-side' -r
complete -c perseus -n "__fish_seen_subcommand_from export-error-page" -l cargo-browser-args -d 'Any arguments to `cargo` when building for the browser-side' -r
complete -c perseus -n "__fish_seen_subcommand_from export-error-page" -l wasm-bindgen-args -d 'Any arguments to `wasm-bindgen`' -r
complete -c perseus -n "__fish_seen_subcommand_from export-error-page" -l wasm-opt-args -d 'Any arguments to `wasm-opt` (only run in release builds)' -r
complete -c perseus -n "__fish_seen_subcommand_from export-error-page" -l git-path -d 'The path to `git` (for downloading custom templates for `perseus new`)' -r
complete -c perseus -n "__fish_seen_subcommand_from export-error-page" -l reload-server-host -d 'The host for the reload server (you should almost never change this)' -r
complete -c perseus -n "__fish_seen_subcommand_from export-error-page" -l reload-server-port -d 'The port for the reload server (you should almost never change this)' -r
complete -c perseus -n "__fish_seen_subcommand_from export-error-page" -l wasm-bindgen-version -d 'A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)' -r
complete -c perseus -n "__fish_seen_subcommand_from export-error-page" -l wasm-opt-version -d 'A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)' -r
complete -c perseus -n "__fish_seen_subcommand_from export-error-page" -l sequential -d 'If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)'
complete -c perseus -n "__fish_seen_subcommand_from export-error-page" -l no-browser-reload -d 'Disable automatic browser reloading'
complete -c perseus -n "__fish_seen_subcommand_from export-error-page" -l no-system-tools-cache -d 'Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)'
complete -c perseus -n "__fish_seen_subcommand_from export-error-page" -l verbose -d 'Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging'
complete -c perseus -n "__fish_seen_subcommand_from export-error-page" -l disable-bundle-compression -d 'Disable Brotli compression of JS and Wasm bundles (may degrade performance)'
complete -c perseus -n "__fish_seen_subcommand_from export-error-page" -s h -l help -d 'Print help'
complete -c perseus -n "__fish_seen_subcommand_from export" -l host -d 'Where to host your exported app' -r
complete -c perseus -n "__fish_seen_subcommand_from export" -l port -d 'The port to host your exported app on' -r
complete -c perseus -n "__fish_seen_subcommand_from export" -l custom-watch -d 'Marks a specific file/directory to be watched (directories will be recursively watched)' -r
complete -c perseus -n "__fish_seen_subcommand_from export" -l cargo-engine-path -d 'The path to `cargo` when used for engine builds' -r
complete -c perseus -n "__fish_seen_subcommand_from export" -l cargo-browser-path -d 'The path to `cargo` when used for browser builds' -r
complete -c perseus -n "__fish_seen_subcommand_from export" -l wasm-bindgen-path -d 'A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)' -r
complete -c perseus -n "__fish_seen_subcommand_from export" -l wasm-opt-path -d 'A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)' -r
complete -c perseus -n "__fish_seen_subcommand_from export" -l rustup-path -d 'The path to `rustup`' -r
complete -c perseus -n "__fish_seen_subcommand_from export" -l wasm-release-rustflags -d 'The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)' -r
complete -c perseus -n "__fish_seen_subcommand_from export" -l cargo-engine-args -d 'Any arguments to `cargo` when building for the engine-side' -r
complete -c perseus -n "__fish_seen_subcommand_from export" -l cargo-browser-args -d 'Any arguments to `cargo` when building for the browser-side' -r
complete -c perseus -n "__fish_seen_subcommand_from export" -l wasm-bindgen-args -d 'Any arguments to `wasm-bindgen`' -r
complete -c perseus -n "__fish_seen_subcommand_from export" -l wasm-opt-args -d 'Any arguments to `wasm-opt` (only run in release builds)' -r
complete -c perseus -n "__fish_seen_subcommand_from export" -l git-path -d 'The path to `git` (for downloading custom templates for `perseus new`)' -r
complete -c perseus -n "__fish_seen_subcommand_from export" -l reload-server-host -d 'The host for the reload server (you should almost never change this)' -r
complete -c perseus -n "__fish_seen_subcommand_from export" -l reload-server-port -d 'The port for the reload server (you should almost never change this)' -r
complete -c perseus -n "__fish_seen_subcommand_from export" -l wasm-bindgen-version -d 'A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)' -r
complete -c perseus -n "__fish_seen_subcommand_from export" -l wasm-opt-version -d 'A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)' -r
complete -c perseus -n "__fish_seen_subcommand_from export" -l release -d 'Export for production'
complete -c perseus -n "__fish_seen_subcommand_from export" -s s -l serve -d 'Serve the generated static files locally'
complete -c perseus -n "__fish_seen_subcommand_from export" -s w -l watch -d 'Watch the files in your working directory for changes (excluding `target/` and `dist/`)'
complete -c perseus -n "__fish_seen_subcommand_from export" -l sequential -d 'If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)'
complete -c perseus -n "__fish_seen_subcommand_from export" -l no-browser-reload -d 'Disable automatic browser reloading'
complete -c perseus -n "__fish_seen_subcommand_from export" -l no-system-tools-cache -d 'Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)'
complete -c perseus -n "__fish_seen_subcommand_from export" -l verbose -d 'Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging'
complete -c perseus -n "__fish_seen_subcommand_from export" -l disable-bundle-compression -d 'Disable Brotli compression of JS and Wasm bundles (may degrade performance)'
complete -c perseus -n "__fish_seen_subcommand_from export" -s h -l help -d 'Print help'
complete -c perseus -n "__fish_seen_subcommand_from serve" -l custom-watch -d 'Marks a specific file/directory to be watched (directories will be recursively watched)' -r
complete -c perseus -n "__fish_seen_subcommand_from serve" -l host -d 'Where to host your exported app' -r
complete -c perseus -n "__fish_seen_subcommand_from serve" -l port -d 'The port to host your exported app on' -r
complete -c perseus -n "__fish_seen_subcommand_from serve" -l cargo-engine-path -d 'The path to `cargo` when used for engine builds' -r
complete -c perseus -n "__fish_seen_subcommand_from serve" -l cargo-browser-path -d 'The path to `cargo` when used for browser builds' -r
complete -c perseus -n "__fish_seen_subcommand_from serve" -l wasm-bindgen-path -d 'A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)' -r
complete -c perseus -n "__fish_seen_subcommand_from serve" -l wasm-opt-path -d 'A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)' -r
complete -c perseus -n "__fish_seen_subcommand_from serve" -l rustup-path -d 'The path to `rustup`' -r
complete -c perseus -n "__fish_seen_subcommand_from serve" -l wasm-release-rustflags -d 'The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)' -r
complete -c perseus -n "__fish_seen_subcommand_from serve" -l cargo-engine-args -d 'Any arguments to `cargo` when building for the engine-side' -r
complete -c perseus -n "__fish_seen_subcommand_from serve" -l cargo-browser-args -d 'Any arguments to `cargo` when building for the browser-side' -r
complete -c perseus -n "__fish_seen_subcommand_from serve" -l wasm-bindgen-args -d 'Any arguments to `wasm-bindgen`' -r
complete -c perseus -n "__fish_seen_subcommand_from serve" -l wasm-opt-args -d 'Any arguments to `wasm-opt` (only run in release builds)' -r
complete -c perseus -n "__fish_seen_subcommand_from serve" -l git-path -d 'The path to `git` (for downloading custom templates for `perseus new`)' -r
complete -c perseus -n "__fish_seen_subcommand_from serve" -l reload-server-host -d 'The host for the reload server (you should almost never change this)' -r
complete -c perseus -n "__fish_seen_subcommand_from serve" -l reload-server-port -d 'The port for the reload server (you should almost never change this)' -r
complete -c perseus -n "__fish_seen_subcommand_from serve" -l wasm-bindgen-version -d 'A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)' -r
complete -c perseus -n "__fish_seen_subcommand_from serve" -l wasm-opt-version -d 'A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)' -r
complete -c perseus -n "__fish_seen_subcommand_from serve" -l no-run -d 'Don\'t run the final binary, but print its location instead as the last line of output'
complete -c perseus -n "__fish_seen_subcommand_from serve" -l no-build -d 'Only build the server, and use the results of a previous `perseus build`'
complete -c perseus -n "__fish_seen_subcommand_from serve" -l release -d 'Build and serve for production'
complete -c perseus -n "__fish_seen_subcommand_from serve" -l standalone -d 'Make the final binary standalone (this is used in `perseus deploy` only, don\'t manually invoke it unless you have a good reason!)'
complete -c perseus -n "__fish_seen_subcommand_from serve" -s w -l watch -d 'Watch the files in your working directory for changes (excluding `target/` and `dist/`)'
complete -c perseus -n "__fish_seen_subcommand_from serve" -l sequential -d 'If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)'
complete -c perseus -n "__fish_seen_subcommand_from serve" -l no-browser-reload -d 'Disable automatic browser reloading'
complete -c perseus -n "__fish_seen_subcommand_from serve" -l no-system-tools-cache -d 'Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)'
complete -c perseus -n "__fish_seen_subcommand_from serve" -l verbose -d 'Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging'
complete -c perseus -n "__fish_seen_subcommand_from serve" -l disable-bundle-compression -d 'Disable Brotli compression of JS and Wasm bundles (may degrade performance)'
complete -c perseus -n "__fish_seen_subcommand_from serve" -s h -l help -d 'Print help'
complete -c perseus -n "__fish_seen_subcommand_from test" -l custom-watch -d 'Marks a specific file/directory to be watched (directories will be recursively watched)' -r
complete -c perseus -n "__fish_seen_subcommand_from test" -l host -d 'Where to host your exported app' -r
complete -c perseus -n "__fish_seen_subcommand_from test" -l port -d 'The port to host your exported app on' -r
complete -c perseus -n "__fish_seen_subcommand_from test" -l cargo-engine-path -d 'The path to `cargo` when used for engine builds' -r
complete -c perseus -n "__fish_seen_subcommand_from test" -l cargo-browser-path -d 'The path to `cargo` when used for browser builds' -r
complete -c perseus -n "__fish_seen_subcommand_from test" -l wasm-bindgen-path -d 'A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)' -r
complete -c perseus -n "__fish_seen_subcommand_from test" -l wasm-opt-path -d 'A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)' -r
complete -c perseus -n "__fish_seen_subcommand_from test" -l rustup-path -d 'The path to `rustup`' -r
complete -c perseus -n "__fish_seen_subcommand_from test" -l wasm-release-rustflags -d 'The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)' -r
complete -c perseus -n "__fish_seen_subcommand_from test" -l cargo-engine-args -d 'Any arguments to `cargo` when building for the engine-side' -r
complete -c perseus -n "__fish_seen_subcommand_from test" -l cargo-browser-args -d 'Any arguments to `cargo` when building for the browser-side' -r
complete -c perseus -n "__fish_seen_subcommand_from test" -l wasm-bindgen-args -d 'Any arguments to `wasm-bindgen`' -r
complete -c perseus -n "__fish_seen_subcommand_from test" -l wasm-opt-args -d 'Any arguments to `wasm-opt` (only run in release builds)' -r
complete -c perseus -n "__fish_seen_subcommand_from test" -l git-path -d 'The path to `git` (for downloading custom templates for `perseus new`)' -r
complete -c perseus -n "__fish_seen_subcommand_from test" -l reload-server-host -d 'The host for the reload server (you should almost never change this)' -r
complete -c perseus -n "__fish_seen_subcommand_from test" -l reload-server-port -d 'The port for the reload server (you should almost never change this)' -r
complete -c perseus -n "__fish_seen_subcommand_from test" -l wasm-bindgen-version -d 'A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)' -r
complete -c perseus -n "__fish_seen_subcommand_from test" -l wasm-opt-version -d 'A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)' -r
complete -c perseus -n "__fish_seen_subcommand_from test" -l no-build -d 'Only build the testing server, and use the results of a previous `perseus build`'
complete -c perseus -n "__fish_seen_subcommand_from test" -l show-browser -d 'Show the browser window when testing (by default, the browser is used in \'headless\' mode); this can be useful for debugging failing tests in some cases'
complete -c perseus -n "__fish_seen_subcommand_from test" -s w -l watch -d 'Watch the files in your working directory for changes (excluding `target/` and `dist/`)'
complete -c perseus -n "__fish_seen_subcommand_from test" -l sequential -d 'If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)'
complete -c perseus -n "__fish_seen_subcommand_from test" -l no-browser-reload -d 'Disable automatic browser reloading'
complete -c perseus -n "__fish_seen_subcommand_from test" -l no-system-tools-cache -d 'Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)'
complete -c perseus -n "__fish_seen_subcommand_from test" -l verbose -d 'Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging'
complete -c perseus -n "__fish_seen_subcommand_from test" -l disable-bundle-compression -d 'Disable Brotli compression of JS and Wasm bundles (may degrade performance)'
complete -c perseus -n "__fish_seen_subcommand_from test" -s h -l help -d 'Print help'
complete -c perseus -n "__fish_seen_subcommand_from clean" -l cargo-engine-path -d 'The path to `cargo` when used for engine builds' -r
complete -c perseus -n "__fish_seen_subcommand_from clean" -l cargo-browser-path -d 'The path to `cargo` when used for browser builds' -r
complete -c perseus -n "__fish_seen_subcommand_from clean" -l wasm-bindgen-path -d 'A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)' -r
complete -c perseus -n "__fish_seen_subcommand_from clean" -l wasm-opt-path -d 'A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)' -r
complete -c perseus -n "__fish_seen_subcommand_from clean" -l rustup-path -d 'The path to `rustup`' -r
complete -c perseus -n "__fish_seen_subcommand_from clean" -l wasm-release-rustflags -d 'The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)' -r
complete -c perseus -n "__fish_seen_subcommand_from clean" -l cargo-engine-args -d 'Any arguments to `cargo` when building for the engine-side' -r
complete -c perseus -n "__fish_seen_subcommand_from clean" -l cargo-browser-args -d 'Any arguments to `cargo` when building for the browser-side' -r
complete -c perseus -n "__fish_seen_subcommand_from clean" -l wasm-bindgen-args -d 'Any arguments to `wasm-bindgen`' -r
complete -c perseus -n "__fish_seen_subcommand_from clean" -l wasm-opt-args -d 'Any arguments to `wasm-opt` (only run in release builds)' -r
complete -c perseus -n "__fish_seen_subcommand_from clean" -l git-path -d 'The path to `git` (for downloading custom templates for `perseus new`)' -r
complete -c perseus -n "__fish_seen_subcommand_from clean" -l reload-server-host -d 'The host for the reload server (you should almost never change this)' -r
complete -c perseus -n "__fish_seen_subcommand_from clean" -l reload-server-port -d 'The port for the reload server (you should almost never change this)' -r
complete -c perseus -n "__fish_seen_subcommand_from clean" -l wasm-bindgen-version -d 'A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)' -r
complete -c perseus -n "__fish_seen_subcommand_from clean" -l wasm-opt-version -d 'A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)' -r
complete -c perseus -n "__fish_seen_subcommand_from clean" -l sequential -d 'If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)'
complete -c perseus -n "__fish_seen_subcommand_from clean" -l no-browser-reload -d 'Disable automatic browser reloading'
complete -c perseus -n "__fish_seen_subcommand_from clean" -l no-system-tools-cache -d 'Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)'
complete -c perseus -n "__fish_seen_subcommand_from clean" -l verbose -d 'Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging'
complete -c perseus -n "__fish_seen_subcommand_from clean" -l disable-bundle-compression -d 'Disable Brotli compression of JS and Wasm bundles (may degrade performance)'
complete -c perseus -n "__fish_seen_subcommand_from clean" -s h -l help -d 'Print help'
complete -c perseus -n "__fish_seen_subcommand_from deploy" -s o -l output -d 'Change the output from `pkg/` to somewhere else' -r
complete -c perseus -n "__fish_seen_subcommand_from deploy" -l cargo-engine-path -d 'The path to `cargo` when used for engine builds' -r
complete -c perseus -n "__fish_seen_subcommand_from deploy" -l cargo-browser-path -d 'The path to `cargo` when used for browser builds' -r
complete -c perseus -n "__fish_seen_subcommand_from deploy" -l wasm-bindgen-path -d 'A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)' -r
complete -c perseus -n "__fish_seen_subcommand_from deploy" -l wasm-opt-path -d 'A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)' -r
complete -c perseus -n "__fish_seen_subcommand_from deploy" -l rustup-path -d 'The path to `rustup`' -r
complete -c perseus -n "__fish_seen_subcommand_from deploy" -l wasm-release-rustflags -d 'The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)' -r
complete -c perseus -n "__fish_seen_subcommand_from deploy" -l cargo-engine-args -d 'Any arguments to `cargo` when building for the engine-side' -r
complete -c perseus -n "__fish_seen_subcommand_from deploy" -l cargo-browser-args -d 'Any arguments to `cargo` when building for the browser-side' -r
complete -c perseus -n "__fish_seen_subcommand_from deploy" -l wasm-bindgen-args -d 'Any arguments to `wasm-bindgen`' -r
complete -c perseus -n "__fish_seen_subcommand_from deploy" -l wasm-opt-args -d 'Any arguments to `wasm-opt` (only run in release builds)' -r
complete -c perseus -n "__fish_seen_subcommand_from deploy" -l git-path -d 'The path to `git` (for downloading custom templates for `perseus new`)' -r
complete -c perseus -n "__fish_seen_subcommand_from deploy" -l reload-server-host -d 'The host for the reload server (you should almost never change this)' -r
complete -c perseus -n "__fish_seen_subcommand_from deploy" -l reload-server-port -d 'The port for the reload server (you should almost never change this)' -r
complete -c perseus -n "__fish_seen_subcommand_from deploy" -l wasm-bindgen-version -d 'A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)' -r
complete -c perseus -n "__fish_seen_subcommand_from deploy" -l wasm-opt-version -d 'A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)' -r
complete -c perseus -n "__fish_seen_subcommand_from deploy" -s e -l export-static -d 'Export your app to purely static files (see `export`)'
complete -c perseus -n "__fish_seen_subcommand_from deploy" -l no-minify-js -d 'Don\'t minify JavaScript (this will decrease performance)'
complete -c perseus -n "__fish_seen_subcommand_from deploy" -l sequential -d 'If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)'
complete -c perseus -n "__fish_seen_subcommand_from deploy" -l no-browser-reload -d 'Disable automatic browser reloading'
complete -c perseus -n "__fish_seen_subcommand_from deploy" -l no-system-tools-cache -d 'Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)'
complete -c perseus -n "__fish_seen_subcommand_from deploy" -l verbose -d 'Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging'
complete -c perseus -n "__fish_seen_subcommand_from deploy" -l disable-bundle-compression -d 'Disable Brotli compression of JS and Wasm bundles (may degrade performance)'
complete -c perseus -n "__fish_seen_subcommand_from deploy" -s h -l help -d 'Print help'
complete -c perseus -n "__fish_seen_subcommand_from tinker" -l cargo-engine-path -d 'The path to `cargo` when used for engine builds' -r
complete -c perseus -n "__fish_seen_subcommand_from tinker" -l cargo-browser-path -d 'The path to `cargo` when used for browser builds' -r
complete -c perseus -n "__fish_seen_subcommand_from tinker" -l wasm-bindgen-path -d 'A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)' -r
complete -c perseus -n "__fish_seen_subcommand_from tinker" -l wasm-opt-path -d 'A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)' -r
complete -c perseus -n "__fish_seen_subcommand_from tinker" -l rustup-path -d 'The path to `rustup`' -r
complete -c perseus -n "__fish_seen_subcommand_from tinker" -l wasm-release-rustflags -d 'The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)' -r
complete -c perseus -n "__fish_seen_subcommand_from tinker" -l cargo-engine-args -d 'Any arguments to `cargo` when building for the engine-side' -r
complete -c perseus -n "__fish_seen_subcommand_from tinker" -l cargo-browser-args -d 'Any arguments to `cargo` when building for the browser-side' -r
complete -c perseus -n "__fish_seen_subcommand_from tinker" -l wasm-bindgen-args -d 'Any arguments to `wasm-bindgen`' -r
complete -c perseus -n "__fish_seen_subcommand_from tinker" -l wasm-opt-args -d 'Any arguments to `wasm-opt` (only run in release builds)' -r
complete -c perseus -n "__fish_seen_subcommand_from tinker" -l git-path -d 'The path to `git` (for downloading custom templates for `perseus new`)' -r
complete -c perseus -n "__fish_seen_subcommand_from tinker" -l reload-server-host -d 'The host for the reload server (you should almost never change this)' -r
complete -c perseus -n "__fish_seen_subcommand_from tinker" -l reload-server-port -d 'The port for the reload server (you should almost never change this)' -r
complete -c perseus -n "__fish_seen_subcommand_from tinker" -l wasm-bindgen-version -d 'A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)' -r
complete -c perseus -n "__fish_seen_subcommand_from tinker" -l wasm-opt-version -d 'A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)' -r
complete -c perseus -n "__fish_seen_subcommand_from tinker" -l no-clean -d 'Don\'t remove and recreate the `dist/` directory'
complete -c perseus -n "__fish_seen_subcommand_from tinker" -l sequential -d 'If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)'
complete -c perseus -n "__fish_seen_subcommand_from tinker" -l no-browser-reload -d 'Disable automatic browser reloading'
complete -c perseus -n "__fish_seen_subcommand_from tinker" -l no-system-tools-cache -d 'Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)'
complete -c perseus -n "__fish_seen_subcommand_from tinker" -l verbose -d 'Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging'
complete -c perseus -n "__fish_seen_subcommand_from tinker" -l disable-bundle-compression -d 'Disable Brotli compression of JS and Wasm bundles (may degrade performance)'
complete -c perseus -n "__fish_seen_subcommand_from tinker" -s h -l help -d 'Print help'
complete -c perseus -n "__fish_seen_subcommand_from snoop; and not __fish_seen_subcommand_from build wasm-build serve help" -l cargo-engine-path -d 'The path to `cargo` when used for engine builds' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop; and not __fish_seen_subcommand_from build wasm-build serve help" -l cargo-browser-path -d 'The path to `cargo` when used for browser builds' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop; and not __fish_seen_subcommand_from build wasm-build serve help" -l wasm-bindgen-path -d 'A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop; and not __fish_seen_subcommand_from build wasm-build serve help" -l wasm-opt-path -d 'A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop; and not __fish_seen_subcommand_from build wasm-build serve help" -l rustup-path -d 'The path to `rustup`' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop; and not __fish_seen_subcommand_from build wasm-build serve help" -l wasm-release-rustflags -d 'The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop; and not __fish_seen_subcommand_from build wasm-build serve help" -l cargo-engine-args -d 'Any arguments to `cargo` when building for the engine-side' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop; and not __fish_seen_subcommand_from build wasm-build serve help" -l cargo-browser-args -d 'Any arguments to `cargo` when building for the browser-side' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop; and not __fish_seen_subcommand_from build wasm-build serve help" -l wasm-bindgen-args -d 'Any arguments to `wasm-bindgen`' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop; and not __fish_seen_subcommand_from build wasm-build serve help" -l wasm-opt-args -d 'Any arguments to `wasm-opt` (only run in release builds)' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop; and not __fish_seen_subcommand_from build wasm-build serve help" -l git-path -d 'The path to `git` (for downloading custom templates for `perseus new`)' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop; and not __fish_seen_subcommand_from build wasm-build serve help" -l reload-server-host -d 'The host for the reload server (you should almost never change this)' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop; and not __fish_seen_subcommand_from build wasm-build serve help" -l reload-server-port -d 'The port for the reload server (you should almost never change this)' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop; and not __fish_seen_subcommand_from build wasm-build serve help" -l wasm-bindgen-version -d 'A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop; and not __fish_seen_subcommand_from build wasm-build serve help" -l wasm-opt-version -d 'A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop; and not __fish_seen_subcommand_from build wasm-build serve help" -l sequential -d 'If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)'
complete -c perseus -n "__fish_seen_subcommand_from snoop; and not __fish_seen_subcommand_from build wasm-build serve help" -l no-browser-reload -d 'Disable automatic browser reloading'
complete -c perseus -n "__fish_seen_subcommand_from snoop; and not __fish_seen_subcommand_from build wasm-build serve help" -l no-system-tools-cache -d 'Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)'
complete -c perseus -n "__fish_seen_subcommand_from snoop; and not __fish_seen_subcommand_from build wasm-build serve help" -l verbose -d 'Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging'
complete -c perseus -n "__fish_seen_subcommand_from snoop; and not __fish_seen_subcommand_from build wasm-build serve help" -l disable-bundle-compression -d 'Disable Brotli compression of JS and Wasm bundles (may degrade performance)'
complete -c perseus -n "__fish_seen_subcommand_from snoop; and not __fish_seen_subcommand_from build wasm-build serve help" -s h -l help -d 'Print help'
complete -c perseus -n "__fish_seen_subcommand_from snoop; and not __fish_seen_subcommand_from build wasm-build serve help" -f -a "build" -d 'Snoops on the static generation process (this will let you see `dbg!` calls and the like)'
complete -c perseus -n "__fish_seen_subcommand_from snoop; and not __fish_seen_subcommand_from build wasm-build serve help" -f -a "wasm-build" -d 'Snoops on the Wasm building process (mostly for debugging errors)'
complete -c perseus -n "__fish_seen_subcommand_from snoop; and not __fish_seen_subcommand_from build wasm-build serve help" -f -a "serve" -d 'Snoops on the server process (run `perseus build` before this)'
complete -c perseus -n "__fish_seen_subcommand_from snoop; and not __fish_seen_subcommand_from build wasm-build serve help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c perseus -n "__fish_seen_subcommand_from snoop build" -l custom-watch -d 'Marks a specific file/directory to be watched (directories will be recursively watched)' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop build" -l cargo-engine-path -d 'The path to `cargo` when used for engine builds' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop build" -l cargo-browser-path -d 'The path to `cargo` when used for browser builds' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop build" -l wasm-bindgen-path -d 'A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop build" -l wasm-opt-path -d 'A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop build" -l rustup-path -d 'The path to `rustup`' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop build" -l wasm-release-rustflags -d 'The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop build" -l cargo-engine-args -d 'Any arguments to `cargo` when building for the engine-side' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop build" -l cargo-browser-args -d 'Any arguments to `cargo` when building for the browser-side' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop build" -l wasm-bindgen-args -d 'Any arguments to `wasm-bindgen`' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop build" -l wasm-opt-args -d 'Any arguments to `wasm-opt` (only run in release builds)' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop build" -l git-path -d 'The path to `git` (for downloading custom templates for `perseus new`)' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop build" -l reload-server-host -d 'The host for the reload server (you should almost never change this)' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop build" -l reload-server-port -d 'The port for the reload server (you should almost never change this)' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop build" -l wasm-bindgen-version -d 'A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop build" -l wasm-opt-version -d 'A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop build" -s w -l watch -d 'Watch the files in your working directory for changes (excluding `target/` and `dist/`)'
complete -c perseus -n "__fish_seen_subcommand_from snoop build" -l sequential -d 'If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)'
complete -c perseus -n "__fish_seen_subcommand_from snoop build" -l no-browser-reload -d 'Disable automatic browser reloading'
complete -c perseus -n "__fish_seen_subcommand_from snoop build" -l no-system-tools-cache -d 'Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)'
complete -c perseus -n "__fish_seen_subcommand_from snoop build" -l verbose -d 'Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging'
complete -c perseus -n "__fish_seen_subcommand_from snoop build" -l disable-bundle-compression -d 'Disable Brotli compression of JS and Wasm bundles (may degrade performance)'
complete -c perseus -n "__fish_seen_subcommand_from snoop build" -s h -l help -d 'Print help'
complete -c perseus -n "__fish_seen_subcommand_from snoop wasm-build" -l custom-watch -d 'Marks a specific file/directory to be watched (directories will be recursively watched)' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop wasm-build" -l cargo-engine-path -d 'The path to `cargo` when used for engine builds' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop wasm-build" -l cargo-browser-path -d 'The path to `cargo` when used for browser builds' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop wasm-build" -l wasm-bindgen-path -d 'A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop wasm-build" -l wasm-opt-path -d 'A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop wasm-build" -l rustup-path -d 'The path to `rustup`' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop wasm-build" -l wasm-release-rustflags -d 'The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop wasm-build" -l cargo-engine-args -d 'Any arguments to `cargo` when building for the engine-side' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop wasm-build" -l cargo-browser-args -d 'Any arguments to `cargo` when building for the browser-side' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop wasm-build" -l wasm-bindgen-args -d 'Any arguments to `wasm-bindgen`' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop wasm-build" -l wasm-opt-args -d 'Any arguments to `wasm-opt` (only run in release builds)' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop wasm-build" -l git-path -d 'The path to `git` (for downloading custom templates for `perseus new`)' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop wasm-build" -l reload-server-host -d 'The host for the reload server (you should almost never change this)' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop wasm-build" -l reload-server-port -d 'The port for the reload server (you should almost never change this)' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop wasm-build" -l wasm-bindgen-version -d 'A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop wasm-build" -l wasm-opt-version -d 'A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop wasm-build" -s w -l watch -d 'Watch the files in your working directory for changes (excluding `target/` and `dist/`)'
complete -c perseus -n "__fish_seen_subcommand_from snoop wasm-build" -l sequential -d 'If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)'
complete -c perseus -n "__fish_seen_subcommand_from snoop wasm-build" -l no-browser-reload -d 'Disable automatic browser reloading'
complete -c perseus -n "__fish_seen_subcommand_from snoop wasm-build" -l no-system-tools-cache -d 'Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)'
complete -c perseus -n "__fish_seen_subcommand_from snoop wasm-build" -l verbose -d 'Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging'
complete -c perseus -n "__fish_seen_subcommand_from snoop wasm-build" -l disable-bundle-compression -d 'Disable Brotli compression of JS and Wasm bundles (may degrade performance)'
complete -c perseus -n "__fish_seen_subcommand_from snoop wasm-build" -s h -l help -d 'Print help'
complete -c perseus -n "__fish_seen_subcommand_from snoop serve" -l host -d 'Where to host your exported app' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop serve" -l port -d 'The port to host your exported app on' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop serve" -l custom-watch -d 'Marks a specific file/directory to be watched (directories will be recursively watched)' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop serve" -l cargo-engine-path -d 'The path to `cargo` when used for engine builds' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop serve" -l cargo-browser-path -d 'The path to `cargo` when used for browser builds' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop serve" -l wasm-bindgen-path -d 'A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop serve" -l wasm-opt-path -d 'A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop serve" -l rustup-path -d 'The path to `rustup`' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop serve" -l wasm-release-rustflags -d 'The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop serve" -l cargo-engine-args -d 'Any arguments to `cargo` when building for the engine-side' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop serve" -l cargo-browser-args -d 'Any arguments to `cargo` when building for the browser-side' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop serve" -l wasm-bindgen-args -d 'Any arguments to `wasm-bindgen`' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop serve" -l wasm-opt-args -d 'Any arguments to `wasm-opt` (only run in release builds)' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop serve" -l git-path -d 'The path to `git` (for downloading custom templates for `perseus new`)' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop serve" -l reload-server-host -d 'The host for the reload server (you should almost never change this)' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop serve" -l reload-server-port -d 'The port for the reload server (you should almost never change this)' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop serve" -l wasm-bindgen-version -d 'A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop serve" -l wasm-opt-version -d 'A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)' -r
complete -c perseus -n "__fish_seen_subcommand_from snoop serve" -s w -l watch -d 'Watch the files in your working directory for changes (excluding `target/` and `dist/`)'
complete -c perseus -n "__fish_seen_subcommand_from snoop serve" -l sequential -d 'If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)'
complete -c perseus -n "__fish_seen_subcommand_from snoop serve" -l no-browser-reload -d 'Disable automatic browser reloading'
complete -c perseus -n "__fish_seen_subcommand_from snoop serve" -l no-system-tools-cache -d 'Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)'
complete -c perseus -n "__fish_seen_subcommand_from snoop serve" -l verbose -d 'Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging'
complete -c perseus -n "__fish_seen_subcommand_from snoop serve" -l disable-bundle-compression -d 'Disable Brotli compression of JS and Wasm bundles (may degrade performance)'
complete -c perseus -n "__fish_seen_subcommand_from snoop serve" -s h -l help -d 'Print help'
complete -c perseus -n "__fish_seen_subcommand_from snoop help; and not __fish_seen_subcommand_from build wasm-build serve help" -f -a "build" -d 'Snoops on the static generation process (this will let you see `dbg!` calls and the like)'
complete -c perseus -n "__fish_seen_subcommand_from snoop help; and not __fish_seen_subcommand_from build wasm-build serve help" -f -a "wasm-build" -d 'Snoops on the Wasm building process (mostly for debugging errors)'
complete -c perseus -n "__fish_seen_subcommand_from snoop help; and not __fish_seen_subcommand_from build wasm-build serve help" -f -a "serve" -d 'Snoops on the server process (run `perseus build` before this)'
complete -c perseus -n "__fish_seen_subcommand_from snoop help; and not __fish_seen_subcommand_from build wasm-build serve help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c perseus -n "__fish_seen_subcommand_from new" -s t -l template -d 'An optional custom URL to a Git repository to be used as a custom template (note that custom templates will not respect your project\'s name). This can be followed with `@branch` to fetch from `branch` rather than the default' -r
complete -c perseus -n "__fish_seen_subcommand_from new" -l dir -d 'The path to a custom directory to create (if this is not provided, the project name will be used by default)' -r
complete -c perseus -n "__fish_seen_subcommand_from new" -l cargo-engine-path -d 'The path to `cargo` when used for engine builds' -r
complete -c perseus -n "__fish_seen_subcommand_from new" -l cargo-browser-path -d 'The path to `cargo` when used for browser builds' -r
complete -c perseus -n "__fish_seen_subcommand_from new" -l wasm-bindgen-path -d 'A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)' -r
complete -c perseus -n "__fish_seen_subcommand_from new" -l wasm-opt-path -d 'A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)' -r
complete -c perseus -n "__fish_seen_subcommand_from new" -l rustup-path -d 'The path to `rustup`' -r
complete -c perseus -n "__fish_seen_subcommand_from new" -l wasm-release-rustflags -d 'The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)' -r
complete -c perseus -n "__fish_seen_subcommand_from new" -l cargo-engine-args -d 'Any arguments to `cargo` when building for the engine-side' -r
complete -c perseus -n "__fish_seen_subcommand_from new" -l cargo-browser-args -d 'Any arguments to `cargo` when building for the browser-side' -r
complete -c perseus -n "__fish_seen_subcommand_from new" -l wasm-bindgen-args -d 'Any arguments to `wasm-bindgen`' -r
complete -c perseus -n "__fish_seen_subcommand_from new" -l wasm-opt-args -d 'Any arguments to `wasm-opt` (only run in release builds)' -r
complete -c perseus -n "__fish_seen_subcommand_from new" -l git-path -d 'The path to `git` (for downloading custom templates for `perseus new`)' -r
complete -c perseus -n "__fish_seen_subcommand_from new" -l reload-server-host -d 'The host for the reload server (you should almost never change this)' -r
complete -c perseus -n "__fish_seen_subcommand_from new" -l reload-server-port -d 'The port for the reload server (you should almost never change this)' -r
complete -c perseus -n "__fish_seen_subcommand_from new" -l wasm-bindgen-version -d 'A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)' -r
complete -c perseus -n "__fish_seen_subcommand_from new" -l wasm-opt-version -d 'A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)' -r
complete -c perseus -n "__fish_seen_subcommand_from new" -l sequential -d 'If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)'
complete -c perseus -n "__fish_seen_subcommand_from new" -l no-browser-reload -d 'Disable automatic browser reloading'
complete -c perseus -n "__fish_seen_subcommand_from new" -l no-system-tools-cache -d 'Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)'
complete -c perseus -n "__fish_seen_subcommand_from new" -l verbose -d 'Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging'
complete -c perseus -n "__fish_seen_subcommand_from new" -l disable-bundle-compression -d 'Disable Brotli compression of JS and Wasm bundles (may degrade performance)'
complete -c perseus -n "__fish_seen_subcommand_from new" -s h -l help -d 'Print help'
complete -c perseus -n "__fish_seen_subcommand_from init" -l cargo-engine-path -d 'The path to `cargo` when used for engine builds' -r
complete -c perseus -n "__fish_seen_subcommand_from init" -l cargo-browser-path -d 'The path to `cargo` when used for browser builds' -r
complete -c perseus -n "__fish_seen_subcommand_from init" -l wasm-bindgen-path -d 'A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)' -r
complete -c perseus -n "__fish_seen_subcommand_from init" -l wasm-opt-path -d 'A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)' -r
complete -c perseus -n "__fish_seen_subcommand_from init" -l rustup-path -d 'The path to `rustup`' -r
complete -c perseus -n "__fish_seen_subcommand_from init" -l wasm-release-rustflags -d 'The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)' -r
complete -c perseus -n "__fish_seen_subcommand_from init" -l cargo-engine-args -d 'Any arguments to `cargo` when building for the engine-side' -r
complete -c perseus -n "__fish_seen_subcommand_from init" -l cargo-browser-args -d 'Any arguments to `cargo` when building for the browser-side' -r
complete -c perseus -n "__fish_seen_subcommand_from init" -l wasm-bindgen-args -d 'Any arguments to `wasm-bindgen`' -r
complete -c perseus -n "__fish_seen_subcommand_from init" -l wasm-opt-args -d 'Any arguments to `wasm-opt` (only run in release builds)' -r
complete -c perseus -n "__fish_seen_subcommand_from init" -l git-path -d 'The path to `git` (for downloading custom templates for `perseus new`)' -r
complete -c perseus -n "__fish_seen_subcommand_from init" -l reload-server-host -d 'The host for the reload server (you should almost never change this)' -r
complete -c perseus -n "__fish_seen_subcommand_from init" -l reload-server-port -d 'The port for the reload server (you should almost never change this)' -r
complete -c perseus -n "__fish_seen_subcommand_from init" -l wasm-bindgen-version -d 'A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)' -r
complete -c perseus -n "__fish_seen_subcommand_from init" -l wasm-opt-version -d 'A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)' -r
complete -c perseus -n "__fish_seen_subcommand_from init" -l sequential -d 'If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)'
complete -c perseus -n "__fish_seen_subcommand_from init" -l no-browser-reload -d 'Disable automatic browser reloading'
complete -c perseus -n "__fish_seen_subcommand_from init" -l no-system-tools-cache -d 'Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)'
complete -c perseus -n "__fish_seen_subcommand_from init" -l verbose -d 'Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging'
complete -c perseus -n "__fish_seen_subcommand_from init" -l disable-bundle-compression -d 'Disable Brotli compression of JS and Wasm bundles (may degrade performance)'
complete -c perseus -n "__fish_seen_subcommand_from init" -s h -l help -d 'Print help'
complete -c perseus -n "__fish_seen_subcommand_from check" -l custom-watch -d 'Marks a specific file/directory to be watched (directories will be recursively watched)' -r
complete -c perseus -n "__fish_seen_subcommand_from check" -l cargo-engine-path -d 'The path to `cargo` when used for engine builds' -r
complete -c perseus -n "__fish_seen_subcommand_from check" -l cargo-browser-path -d 'The path to `cargo` when used for browser builds' -r
complete -c perseus -n "__fish_seen_subcommand_from check" -l wasm-bindgen-path -d 'A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)' -r
complete -c perseus -n "__fish_seen_subcommand_from check" -l wasm-opt-path -d 'A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)' -r
complete -c perseus -n "__fish_seen_subcommand_from check" -l rustup-path -d 'The path to `rustup`' -r
complete -c perseus -n "__fish_seen_subcommand_from check" -l wasm-release-rustflags -d 'The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)' -r
complete -c perseus -n "__fish_seen_subcommand_from check" -l cargo-engine-args -d 'Any arguments to `cargo` when building for the engine-side' -r
complete -c perseus -n "__fish_seen_subcommand_from check" -l cargo-browser-args -d 'Any arguments to `cargo` when building for the browser-side' -r
complete -c perseus -n "__fish_seen_subcommand_from check" -l wasm-bindgen-args -d 'Any arguments to `wasm-bindgen`' -r
complete -c perseus -n "__fish_seen_subcommand_from check" -l wasm-opt-args -d 'Any arguments to `wasm-opt` (only run in release builds)' -r
complete -c perseus -n "__fish_seen_subcommand_from check" -l git-path -d 'The path to `git` (for downloading custom templates for `perseus new`)' -r
complete -c perseus -n "__fish_seen_subcommand_from check" -l reload-server-host -d 'The host for the reload server (you should almost never change this)' -r
complete -c perseus -n "__fish_seen_subcommand_from check" -l reload-server-port -d 'The port for the reload server (you should almost never change this)' -r
complete -c perseus -n "__fish_seen_subcommand_from check" -l wasm-bindgen-version -d 'A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)' -r
complete -c perseus -n "__fish_seen_subcommand_from check" -l wasm-opt-version -d 'A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)' -r
complete -c perseus -n "__fish_seen_subcommand_from check" -s w -l watch -d 'Watch the files in your working directory for changes (excluding `target/` and `dist/`)'
complete -c perseus -n "__fish_seen_subcommand_from check" -s g -l generate -d 'Make sure the app\'s page generation works properly (this will take much longer, but almost guarantees that your app will actually build); use this to catch errors in build state and the like'
complete -c perseus -n "__fish_seen_subcommand_from check" -l sequential -d 'If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)'
complete -c perseus -n "__fish_seen_subcommand_from check" -l no-browser-reload -d 'Disable automatic browser reloading'
complete -c perseus -n "__fish_seen_subcommand_from check" -l no-system-tools-cache -d 'Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)'
complete -c perseus -n "__fish_seen_subcommand_from check" -l verbose -d 'Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging'
complete -c perseus -n "__fish_seen_subcommand_from check" -l disable-bundle-compression -d 'Disable Brotli compression of JS and Wasm bundles (may degrade performance)'
complete -c perseus -n "__fish_seen_subcommand_from check" -s h -l help -d 'Print help'
complete -c perseus -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from build export-error-page export serve test clean deploy tinker snoop new init check help" -f -a "build" -d 'Builds your app'
complete -c perseus -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from build export-error-page export serve test clean deploy tinker snoop new init check help" -f -a "export-error-page" -d 'Exports an error page for the given HTTP status code'
complete -c perseus -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from build export-error-page export serve test clean deploy tinker snoop new init check help" -f -a "export" -d 'Exports your app to purely static files'
complete -c perseus -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from build export-error-page export serve test clean deploy tinker snoop new init check help" -f -a "serve" -d 'Serves your app'
complete -c perseus -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from build export-error-page export serve test clean deploy tinker snoop new init check help" -f -a "test" -d 'Serves your app as `perseus serve` does, but puts it in testing mode'
complete -c perseus -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from build export-error-page export serve test clean deploy tinker snoop new init check help" -f -a "clean" -d 'Removes build artifacts in the `dist/` directory'
complete -c perseus -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from build export-error-page export serve test clean deploy tinker snoop new init check help" -f -a "deploy" -d 'Packages your app for deployment'
complete -c perseus -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from build export-error-page export serve test clean deploy tinker snoop new init check help" -f -a "tinker" -d 'Runs the `tinker` action of plugins, which lets them modify the Perseus engine'
complete -c perseus -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from build export-error-page export serve test clean deploy tinker snoop new init check help" -f -a "snoop" -d 'Runs one of the underlying commands that builds your app, allowing you to see more detailed logs'
complete -c perseus -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from build export-error-page export serve test clean deploy tinker snoop new init check help" -f -a "new" -d 'Creates a new Perseus project in a directory of the given name, which will be created in the current path'
complete -c perseus -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from build export-error-page export serve test clean deploy tinker snoop new init check help" -f -a "init" -d 'Initializes a new Perseus project in the current directory'
complete -c perseus -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from build export-error-page export serve test clean deploy tinker snoop new init check help" -f -a "check" -d 'Checks if your app builds properly for both the engine-side and the browser-side'
complete -c perseus -n "__fish_seen_subcommand_from help; and not __fish_seen_subcommand_from build export-error-page export serve test clean deploy tinker snoop new init check help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c perseus -n "__fish_seen_subcommand_from help snoop; and not __fish_seen_subcommand_from build wasm-build serve" -f -a "build" -d 'Snoops on the static generation process (this will let you see `dbg!` calls and the like)'
complete -c perseus -n "__fish_seen_subcommand_from help snoop; and not __fish_seen_subcommand_from build wasm-build serve" -f -a "wasm-build" -d 'Snoops on the Wasm building process (mostly for debugging errors)'
complete -c perseus -n "__fish_seen_subcommand_from help snoop; and not __fish_seen_subcommand_from build wasm-build serve" -f -a "serve" -d 'Snoops on the server process (run `perseus build` before this)'
