
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'perseus' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'perseus'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'perseus' {
            [CompletionResult]::new('--cargo-engine-path', 'cargo-engine-path', [CompletionResultType]::ParameterName, 'The path to `cargo` when used for engine builds')
            [CompletionResult]::new('--cargo-browser-path', 'cargo-browser-path', [CompletionResultType]::ParameterName, 'The path to `cargo` when used for browser builds')
            [CompletionResult]::new('--wasm-bindgen-path', 'wasm-bindgen-path', [CompletionResultType]::ParameterName, 'A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)')
            [CompletionResult]::new('--wasm-opt-path', 'wasm-opt-path', [CompletionResultType]::ParameterName, 'A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)')
            [CompletionResult]::new('--rustup-path', 'rustup-path', [CompletionResultType]::ParameterName, 'The path to `rustup`')
            [CompletionResult]::new('--wasm-release-rustflags', 'wasm-release-rustflags', [CompletionResultType]::ParameterName, 'The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)')
            [CompletionResult]::new('--cargo-engine-args', 'cargo-engine-args', [CompletionResultType]::ParameterName, 'Any arguments to `cargo` when building for the engine-side')
            [CompletionResult]::new('--cargo-browser-args', 'cargo-browser-args', [CompletionResultType]::ParameterName, 'Any arguments to `cargo` when building for the browser-side')
            [CompletionResult]::new('--wasm-bindgen-args', 'wasm-bindgen-args', [CompletionResultType]::ParameterName, 'Any arguments to `wasm-bindgen`')
            [CompletionResult]::new('--wasm-opt-args', 'wasm-opt-args', [CompletionResultType]::ParameterName, 'Any arguments to `wasm-opt` (only run in release builds)')
            [CompletionResult]::new('--git-path', 'git-path', [CompletionResultType]::ParameterName, 'The path to `git` (for downloading custom templates for `perseus new`)')
            [CompletionResult]::new('--reload-server-host', 'reload-server-host', [CompletionResultType]::ParameterName, 'The host for the reload server (you should almost never change this)')
            [CompletionResult]::new('--reload-server-port', 'reload-server-port', [CompletionResultType]::ParameterName, 'The port for the reload server (you should almost never change this)')
            [CompletionResult]::new('--wasm-bindgen-version', 'wasm-bindgen-version', [CompletionResultType]::ParameterName, 'A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)')
            [CompletionResult]::new('--wasm-opt-version', 'wasm-opt-version', [CompletionResultType]::ParameterName, 'A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)')
            [CompletionResult]::new('--sequential', 'sequential', [CompletionResultType]::ParameterName, 'If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)')
            [CompletionResult]::new('--no-browser-reload', 'no-browser-reload', [CompletionResultType]::ParameterName, 'Disable automatic browser reloading')
            [CompletionResult]::new('--no-system-tools-cache', 'no-system-tools-cache', [CompletionResultType]::ParameterName, 'Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging')
            [CompletionResult]::new('--disable-bundle-compression', 'disable-bundle-compression', [CompletionResultType]::ParameterName, 'Disable Brotli compression of JS and Wasm bundles (may degrade performance)')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', 'V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('build', 'build', [CompletionResultType]::ParameterValue, 'Builds your app')
            [CompletionResult]::new('export-error-page', 'export-error-page', [CompletionResultType]::ParameterValue, 'Exports an error page for the given HTTP status code')
            [CompletionResult]::new('export', 'export', [CompletionResultType]::ParameterValue, 'Exports your app to purely static files')
            [CompletionResult]::new('serve', 'serve', [CompletionResultType]::ParameterValue, 'Serves your app')
            [CompletionResult]::new('test', 'test', [CompletionResultType]::ParameterValue, 'Serves your app as `perseus serve` does, but puts it in testing mode')
            [CompletionResult]::new('clean', 'clean', [CompletionResultType]::ParameterValue, 'Removes build artifacts in the `dist/` directory')
            [CompletionResult]::new('deploy', 'deploy', [CompletionResultType]::ParameterValue, 'Packages your app for deployment')
            [CompletionResult]::new('tinker', 'tinker', [CompletionResultType]::ParameterValue, 'Runs the `tinker` action of plugins, which lets them modify the Perseus engine')
            [CompletionResult]::new('snoop', 'snoop', [CompletionResultType]::ParameterValue, 'Runs one of the underlying commands that builds your app, allowing you to see more detailed logs')
            [CompletionResult]::new('new', 'new', [CompletionResultType]::ParameterValue, 'Creates a new Perseus project in a directory of the given name, which will be created in the current path')
            [CompletionResult]::new('init', 'init', [CompletionResultType]::ParameterValue, 'Initializes a new Perseus project in the current directory')
            [CompletionResult]::new('check', 'check', [CompletionResultType]::ParameterValue, 'Checks if your app builds properly for both the engine-side and the browser-side')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'perseus;build' {
            [CompletionResult]::new('--custom-watch', 'custom-watch', [CompletionResultType]::ParameterName, 'Marks a specific file/directory to be watched (directories will be recursively watched)')
            [CompletionResult]::new('--cargo-engine-path', 'cargo-engine-path', [CompletionResultType]::ParameterName, 'The path to `cargo` when used for engine builds')
            [CompletionResult]::new('--cargo-browser-path', 'cargo-browser-path', [CompletionResultType]::ParameterName, 'The path to `cargo` when used for browser builds')
            [CompletionResult]::new('--wasm-bindgen-path', 'wasm-bindgen-path', [CompletionResultType]::ParameterName, 'A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)')
            [CompletionResult]::new('--wasm-opt-path', 'wasm-opt-path', [CompletionResultType]::ParameterName, 'A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)')
            [CompletionResult]::new('--rustup-path', 'rustup-path', [CompletionResultType]::ParameterName, 'The path to `rustup`')
            [CompletionResult]::new('--wasm-release-rustflags', 'wasm-release-rustflags', [CompletionResultType]::ParameterName, 'The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)')
            [CompletionResult]::new('--cargo-engine-args', 'cargo-engine-args', [CompletionResultType]::ParameterName, 'Any arguments to `cargo` when building for the engine-side')
            [CompletionResult]::new('--cargo-browser-args', 'cargo-browser-args', [CompletionResultType]::ParameterName, 'Any arguments to `cargo` when building for the browser-side')
            [CompletionResult]::new('--wasm-bindgen-args', 'wasm-bindgen-args', [CompletionResultType]::ParameterName, 'Any arguments to `wasm-bindgen`')
            [CompletionResult]::new('--wasm-opt-args', 'wasm-opt-args', [CompletionResultType]::ParameterName, 'Any arguments to `wasm-opt` (only run in release builds)')
            [CompletionResult]::new('--git-path', 'git-path', [CompletionResultType]::ParameterName, 'The path to `git` (for downloading custom templates for `perseus new`)')
            [CompletionResult]::new('--reload-server-host', 'reload-server-host', [CompletionResultType]::ParameterName, 'The host for the reload server (you should almost never change this)')
            [CompletionResult]::new('--reload-server-port', 'reload-server-port', [CompletionResultType]::ParameterName, 'The port for the reload server (you should almost never change this)')
            [CompletionResult]::new('--wasm-bindgen-version', 'wasm-bindgen-version', [CompletionResultType]::ParameterName, 'A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)')
            [CompletionResult]::new('--wasm-opt-version', 'wasm-opt-version', [CompletionResultType]::ParameterName, 'A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)')
            [CompletionResult]::new('--release', 'release', [CompletionResultType]::ParameterName, 'Build for production')
            [CompletionResult]::new('-w', 'w', [CompletionResultType]::ParameterName, 'Watch the files in your working directory for changes (excluding `target/` and `dist/`)')
            [CompletionResult]::new('--watch', 'watch', [CompletionResultType]::ParameterName, 'Watch the files in your working directory for changes (excluding `target/` and `dist/`)')
            [CompletionResult]::new('--sequential', 'sequential', [CompletionResultType]::ParameterName, 'If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)')
            [CompletionResult]::new('--no-browser-reload', 'no-browser-reload', [CompletionResultType]::ParameterName, 'Disable automatic browser reloading')
            [CompletionResult]::new('--no-system-tools-cache', 'no-system-tools-cache', [CompletionResultType]::ParameterName, 'Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging')
            [CompletionResult]::new('--disable-bundle-compression', 'disable-bundle-compression', [CompletionResultType]::ParameterName, 'Disable Brotli compression of JS and Wasm bundles (may degrade performance)')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'perseus;export-error-page' {
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'c')
            [CompletionResult]::new('--code', 'code', [CompletionResultType]::ParameterName, 'code')
            [CompletionResult]::new('-o', 'o', [CompletionResultType]::ParameterName, 'o')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--cargo-engine-path', 'cargo-engine-path', [CompletionResultType]::ParameterName, 'The path to `cargo` when used for engine builds')
            [CompletionResult]::new('--cargo-browser-path', 'cargo-browser-path', [CompletionResultType]::ParameterName, 'The path to `cargo` when used for browser builds')
            [CompletionResult]::new('--wasm-bindgen-path', 'wasm-bindgen-path', [CompletionResultType]::ParameterName, 'A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)')
            [CompletionResult]::new('--wasm-opt-path', 'wasm-opt-path', [CompletionResultType]::ParameterName, 'A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)')
            [CompletionResult]::new('--rustup-path', 'rustup-path', [CompletionResultType]::ParameterName, 'The path to `rustup`')
            [CompletionResult]::new('--wasm-release-rustflags', 'wasm-release-rustflags', [CompletionResultType]::ParameterName, 'The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)')
            [CompletionResult]::new('--cargo-engine-args', 'cargo-engine-args', [CompletionResultType]::ParameterName, 'Any arguments to `cargo` when building for the engine-side')
            [CompletionResult]::new('--cargo-browser-args', 'cargo-browser-args', [CompletionResultType]::ParameterName, 'Any arguments to `cargo` when building for the browser-side')
            [CompletionResult]::new('--wasm-bindgen-args', 'wasm-bindgen-args', [CompletionResultType]::ParameterName, 'Any arguments to `wasm-bindgen`')
            [CompletionResult]::new('--wasm-opt-args', 'wasm-opt-args', [CompletionResultType]::ParameterName, 'Any arguments to `wasm-opt` (only run in release builds)')
            [CompletionResult]::new('--git-path', 'git-path', [CompletionResultType]::ParameterName, 'The path to `git` (for downloading custom templates for `perseus new`)')
            [CompletionResult]::new('--reload-server-host', 'reload-server-host', [CompletionResultType]::ParameterName, 'The host for the reload server (you should almost never change this)')
            [CompletionResult]::new('--reload-server-port', 'reload-server-port', [CompletionResultType]::ParameterName, 'The port for the reload server (you should almost never change this)')
            [CompletionResult]::new('--wasm-bindgen-version', 'wasm-bindgen-version', [CompletionResultType]::ParameterName, 'A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)')
            [CompletionResult]::new('--wasm-opt-version', 'wasm-opt-version', [CompletionResultType]::ParameterName, 'A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)')
            [CompletionResult]::new('--sequential', 'sequential', [CompletionResultType]::ParameterName, 'If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)')
            [CompletionResult]::new('--no-browser-reload', 'no-browser-reload', [CompletionResultType]::ParameterName, 'Disable automatic browser reloading')
            [CompletionResult]::new('--no-system-tools-cache', 'no-system-tools-cache', [CompletionResultType]::ParameterName, 'Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging')
            [CompletionResult]::new('--disable-bundle-compression', 'disable-bundle-compression', [CompletionResultType]::ParameterName, 'Disable Brotli compression of JS and Wasm bundles (may degrade performance)')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'perseus;export' {
            [CompletionResult]::new('--host', 'host', [CompletionResultType]::ParameterName, 'Where to host your exported app')
            [CompletionResult]::new('--port', 'port', [CompletionResultType]::ParameterName, 'The port to host your exported app on')
            [CompletionResult]::new('--custom-watch', 'custom-watch', [CompletionResultType]::ParameterName, 'Marks a specific file/directory to be watched (directories will be recursively watched)')
            [CompletionResult]::new('--cargo-engine-path', 'cargo-engine-path', [CompletionResultType]::ParameterName, 'The path to `cargo` when used for engine builds')
            [CompletionResult]::new('--cargo-browser-path', 'cargo-browser-path', [CompletionResultType]::ParameterName, 'The path to `cargo` when used for browser builds')
            [CompletionResult]::new('--wasm-bindgen-path', 'wasm-bindgen-path', [CompletionResultType]::ParameterName, 'A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)')
            [CompletionResult]::new('--wasm-opt-path', 'wasm-opt-path', [CompletionResultType]::ParameterName, 'A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)')
            [CompletionResult]::new('--rustup-path', 'rustup-path', [CompletionResultType]::ParameterName, 'The path to `rustup`')
            [CompletionResult]::new('--wasm-release-rustflags', 'wasm-release-rustflags', [CompletionResultType]::ParameterName, 'The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)')
            [CompletionResult]::new('--cargo-engine-args', 'cargo-engine-args', [CompletionResultType]::ParameterName, 'Any arguments to `cargo` when building for the engine-side')
            [CompletionResult]::new('--cargo-browser-args', 'cargo-browser-args', [CompletionResultType]::ParameterName, 'Any arguments to `cargo` when building for the browser-side')
            [CompletionResult]::new('--wasm-bindgen-args', 'wasm-bindgen-args', [CompletionResultType]::ParameterName, 'Any arguments to `wasm-bindgen`')
            [CompletionResult]::new('--wasm-opt-args', 'wasm-opt-args', [CompletionResultType]::ParameterName, 'Any arguments to `wasm-opt` (only run in release builds)')
            [CompletionResult]::new('--git-path', 'git-path', [CompletionResultType]::ParameterName, 'The path to `git` (for downloading custom templates for `perseus new`)')
            [CompletionResult]::new('--reload-server-host', 'reload-server-host', [CompletionResultType]::ParameterName, 'The host for the reload server (you should almost never change this)')
            [CompletionResult]::new('--reload-server-port', 'reload-server-port', [CompletionResultType]::ParameterName, 'The port for the reload server (you should almost never change this)')
            [CompletionResult]::new('--wasm-bindgen-version', 'wasm-bindgen-version', [CompletionResultType]::ParameterName, 'A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)')
            [CompletionResult]::new('--wasm-opt-version', 'wasm-opt-version', [CompletionResultType]::ParameterName, 'A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)')
            [CompletionResult]::new('--release', 'release', [CompletionResultType]::ParameterName, 'Export for production')
            [CompletionResult]::new('-s', 's', [CompletionResultType]::ParameterName, 'Serve the generated static files locally')
            [CompletionResult]::new('--serve', 'serve', [CompletionResultType]::ParameterName, 'Serve the generated static files locally')
            [CompletionResult]::new('-w', 'w', [CompletionResultType]::ParameterName, 'Watch the files in your working directory for changes (excluding `target/` and `dist/`)')
            [CompletionResult]::new('--watch', 'watch', [CompletionResultType]::ParameterName, 'Watch the files in your working directory for changes (excluding `target/` and `dist/`)')
            [CompletionResult]::new('--sequential', 'sequential', [CompletionResultType]::ParameterName, 'If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)')
            [CompletionResult]::new('--no-browser-reload', 'no-browser-reload', [CompletionResultType]::ParameterName, 'Disable automatic browser reloading')
            [CompletionResult]::new('--no-system-tools-cache', 'no-system-tools-cache', [CompletionResultType]::ParameterName, 'Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging')
            [CompletionResult]::new('--disable-bundle-compression', 'disable-bundle-compression', [CompletionResultType]::ParameterName, 'Disable Brotli compression of JS and Wasm bundles (may degrade performance)')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'perseus;serve' {
            [CompletionResult]::new('--custom-watch', 'custom-watch', [CompletionResultType]::ParameterName, 'Marks a specific file/directory to be watched (directories will be recursively watched)')
            [CompletionResult]::new('--host', 'host', [CompletionResultType]::ParameterName, 'Where to host your exported app')
            [CompletionResult]::new('--port', 'port', [CompletionResultType]::ParameterName, 'The port to host your exported app on')
            [CompletionResult]::new('--cargo-engine-path', 'cargo-engine-path', [CompletionResultType]::ParameterName, 'The path to `cargo` when used for engine builds')
            [CompletionResult]::new('--cargo-browser-path', 'cargo-browser-path', [CompletionResultType]::ParameterName, 'The path to `cargo` when used for browser builds')
            [CompletionResult]::new('--wasm-bindgen-path', 'wasm-bindgen-path', [CompletionResultType]::ParameterName, 'A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)')
            [CompletionResult]::new('--wasm-opt-path', 'wasm-opt-path', [CompletionResultType]::ParameterName, 'A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)')
            [CompletionResult]::new('--rustup-path', 'rustup-path', [CompletionResultType]::ParameterName, 'The path to `rustup`')
            [CompletionResult]::new('--wasm-release-rustflags', 'wasm-release-rustflags', [CompletionResultType]::ParameterName, 'The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)')
            [CompletionResult]::new('--cargo-engine-args', 'cargo-engine-args', [CompletionResultType]::ParameterName, 'Any arguments to `cargo` when building for the engine-side')
            [CompletionResult]::new('--cargo-browser-args', 'cargo-browser-args', [CompletionResultType]::ParameterName, 'Any arguments to `cargo` when building for the browser-side')
            [CompletionResult]::new('--wasm-bindgen-args', 'wasm-bindgen-args', [CompletionResultType]::ParameterName, 'Any arguments to `wasm-bindgen`')
            [CompletionResult]::new('--wasm-opt-args', 'wasm-opt-args', [CompletionResultType]::ParameterName, 'Any arguments to `wasm-opt` (only run in release builds)')
            [CompletionResult]::new('--git-path', 'git-path', [CompletionResultType]::ParameterName, 'The path to `git` (for downloading custom templates for `perseus new`)')
            [CompletionResult]::new('--reload-server-host', 'reload-server-host', [CompletionResultType]::ParameterName, 'The host for the reload server (you should almost never change this)')
            [CompletionResult]::new('--reload-server-port', 'reload-server-port', [CompletionResultType]::ParameterName, 'The port for the reload server (you should almost never change this)')
            [CompletionResult]::new('--wasm-bindgen-version', 'wasm-bindgen-version', [CompletionResultType]::ParameterName, 'A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)')
            [CompletionResult]::new('--wasm-opt-version', 'wasm-opt-version', [CompletionResultType]::ParameterName, 'A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)')
            [CompletionResult]::new('--no-run', 'no-run', [CompletionResultType]::ParameterName, 'Don''t run the final binary, but print its location instead as the last line of output')
            [CompletionResult]::new('--no-build', 'no-build', [CompletionResultType]::ParameterName, 'Only build the server, and use the results of a previous `perseus build`')
            [CompletionResult]::new('--release', 'release', [CompletionResultType]::ParameterName, 'Build and serve for production')
            [CompletionResult]::new('--standalone', 'standalone', [CompletionResultType]::ParameterName, 'Make the final binary standalone (this is used in `perseus deploy` only, don''t manually invoke it unless you have a good reason!)')
            [CompletionResult]::new('-w', 'w', [CompletionResultType]::ParameterName, 'Watch the files in your working directory for changes (excluding `target/` and `dist/`)')
            [CompletionResult]::new('--watch', 'watch', [CompletionResultType]::ParameterName, 'Watch the files in your working directory for changes (excluding `target/` and `dist/`)')
            [CompletionResult]::new('--sequential', 'sequential', [CompletionResultType]::ParameterName, 'If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)')
            [CompletionResult]::new('--no-browser-reload', 'no-browser-reload', [CompletionResultType]::ParameterName, 'Disable automatic browser reloading')
            [CompletionResult]::new('--no-system-tools-cache', 'no-system-tools-cache', [CompletionResultType]::ParameterName, 'Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging')
            [CompletionResult]::new('--disable-bundle-compression', 'disable-bundle-compression', [CompletionResultType]::ParameterName, 'Disable Brotli compression of JS and Wasm bundles (may degrade performance)')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'perseus;test' {
            [CompletionResult]::new('--custom-watch', 'custom-watch', [CompletionResultType]::ParameterName, 'Marks a specific file/directory to be watched (directories will be recursively watched)')
            [CompletionResult]::new('--host', 'host', [CompletionResultType]::ParameterName, 'Where to host your exported app')
            [CompletionResult]::new('--port', 'port', [CompletionResultType]::ParameterName, 'The port to host your exported app on')
            [CompletionResult]::new('--cargo-engine-path', 'cargo-engine-path', [CompletionResultType]::ParameterName, 'The path to `cargo` when used for engine builds')
            [CompletionResult]::new('--cargo-browser-path', 'cargo-browser-path', [CompletionResultType]::ParameterName, 'The path to `cargo` when used for browser builds')
            [CompletionResult]::new('--wasm-bindgen-path', 'wasm-bindgen-path', [CompletionResultType]::ParameterName, 'A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)')
            [CompletionResult]::new('--wasm-opt-path', 'wasm-opt-path', [CompletionResultType]::ParameterName, 'A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)')
            [CompletionResult]::new('--rustup-path', 'rustup-path', [CompletionResultType]::ParameterName, 'The path to `rustup`')
            [CompletionResult]::new('--wasm-release-rustflags', 'wasm-release-rustflags', [CompletionResultType]::ParameterName, 'The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)')
            [CompletionResult]::new('--cargo-engine-args', 'cargo-engine-args', [CompletionResultType]::ParameterName, 'Any arguments to `cargo` when building for the engine-side')
            [CompletionResult]::new('--cargo-browser-args', 'cargo-browser-args', [CompletionResultType]::ParameterName, 'Any arguments to `cargo` when building for the browser-side')
            [CompletionResult]::new('--wasm-bindgen-args', 'wasm-bindgen-args', [CompletionResultType]::ParameterName, 'Any arguments to `wasm-bindgen`')
            [CompletionResult]::new('--wasm-opt-args', 'wasm-opt-args', [CompletionResultType]::ParameterName, 'Any arguments to `wasm-opt` (only run in release builds)')
            [CompletionResult]::new('--git-path', 'git-path', [CompletionResultType]::ParameterName, 'The path to `git` (for downloading custom templates for `perseus new`)')
            [CompletionResult]::new('--reload-server-host', 'reload-server-host', [CompletionResultType]::ParameterName, 'The host for the reload server (you should almost never change this)')
            [CompletionResult]::new('--reload-server-port', 'reload-server-port', [CompletionResultType]::ParameterName, 'The port for the reload server (you should almost never change this)')
            [CompletionResult]::new('--wasm-bindgen-version', 'wasm-bindgen-version', [CompletionResultType]::ParameterName, 'A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)')
            [CompletionResult]::new('--wasm-opt-version', 'wasm-opt-version', [CompletionResultType]::ParameterName, 'A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)')
            [CompletionResult]::new('--no-build', 'no-build', [CompletionResultType]::ParameterName, 'Only build the testing server, and use the results of a previous `perseus build`')
            [CompletionResult]::new('--show-browser', 'show-browser', [CompletionResultType]::ParameterName, 'Show the browser window when testing (by default, the browser is used in ''headless'' mode); this can be useful for debugging failing tests in some cases')
            [CompletionResult]::new('-w', 'w', [CompletionResultType]::ParameterName, 'Watch the files in your working directory for changes (excluding `target/` and `dist/`)')
            [CompletionResult]::new('--watch', 'watch', [CompletionResultType]::ParameterName, 'Watch the files in your working directory for changes (excluding `target/` and `dist/`)')
            [CompletionResult]::new('--sequential', 'sequential', [CompletionResultType]::ParameterName, 'If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)')
            [CompletionResult]::new('--no-browser-reload', 'no-browser-reload', [CompletionResultType]::ParameterName, 'Disable automatic browser reloading')
            [CompletionResult]::new('--no-system-tools-cache', 'no-system-tools-cache', [CompletionResultType]::ParameterName, 'Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging')
            [CompletionResult]::new('--disable-bundle-compression', 'disable-bundle-compression', [CompletionResultType]::ParameterName, 'Disable Brotli compression of JS and Wasm bundles (may degrade performance)')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'perseus;clean' {
            [CompletionResult]::new('--cargo-engine-path', 'cargo-engine-path', [CompletionResultType]::ParameterName, 'The path to `cargo` when used for engine builds')
            [CompletionResult]::new('--cargo-browser-path', 'cargo-browser-path', [CompletionResultType]::ParameterName, 'The path to `cargo` when used for browser builds')
            [CompletionResult]::new('--wasm-bindgen-path', 'wasm-bindgen-path', [CompletionResultType]::ParameterName, 'A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)')
            [CompletionResult]::new('--wasm-opt-path', 'wasm-opt-path', [CompletionResultType]::ParameterName, 'A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)')
            [CompletionResult]::new('--rustup-path', 'rustup-path', [CompletionResultType]::ParameterName, 'The path to `rustup`')
            [CompletionResult]::new('--wasm-release-rustflags', 'wasm-release-rustflags', [CompletionResultType]::ParameterName, 'The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)')
            [CompletionResult]::new('--cargo-engine-args', 'cargo-engine-args', [CompletionResultType]::ParameterName, 'Any arguments to `cargo` when building for the engine-side')
            [CompletionResult]::new('--cargo-browser-args', 'cargo-browser-args', [CompletionResultType]::ParameterName, 'Any arguments to `cargo` when building for the browser-side')
            [CompletionResult]::new('--wasm-bindgen-args', 'wasm-bindgen-args', [CompletionResultType]::ParameterName, 'Any arguments to `wasm-bindgen`')
            [CompletionResult]::new('--wasm-opt-args', 'wasm-opt-args', [CompletionResultType]::ParameterName, 'Any arguments to `wasm-opt` (only run in release builds)')
            [CompletionResult]::new('--git-path', 'git-path', [CompletionResultType]::ParameterName, 'The path to `git` (for downloading custom templates for `perseus new`)')
            [CompletionResult]::new('--reload-server-host', 'reload-server-host', [CompletionResultType]::ParameterName, 'The host for the reload server (you should almost never change this)')
            [CompletionResult]::new('--reload-server-port', 'reload-server-port', [CompletionResultType]::ParameterName, 'The port for the reload server (you should almost never change this)')
            [CompletionResult]::new('--wasm-bindgen-version', 'wasm-bindgen-version', [CompletionResultType]::ParameterName, 'A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)')
            [CompletionResult]::new('--wasm-opt-version', 'wasm-opt-version', [CompletionResultType]::ParameterName, 'A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)')
            [CompletionResult]::new('--sequential', 'sequential', [CompletionResultType]::ParameterName, 'If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)')
            [CompletionResult]::new('--no-browser-reload', 'no-browser-reload', [CompletionResultType]::ParameterName, 'Disable automatic browser reloading')
            [CompletionResult]::new('--no-system-tools-cache', 'no-system-tools-cache', [CompletionResultType]::ParameterName, 'Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging')
            [CompletionResult]::new('--disable-bundle-compression', 'disable-bundle-compression', [CompletionResultType]::ParameterName, 'Disable Brotli compression of JS and Wasm bundles (may degrade performance)')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'perseus;deploy' {
            [CompletionResult]::new('-o', 'o', [CompletionResultType]::ParameterName, 'Change the output from `pkg/` to somewhere else')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'Change the output from `pkg/` to somewhere else')
            [CompletionResult]::new('--cargo-engine-path', 'cargo-engine-path', [CompletionResultType]::ParameterName, 'The path to `cargo` when used for engine builds')
            [CompletionResult]::new('--cargo-browser-path', 'cargo-browser-path', [CompletionResultType]::ParameterName, 'The path to `cargo` when used for browser builds')
            [CompletionResult]::new('--wasm-bindgen-path', 'wasm-bindgen-path', [CompletionResultType]::ParameterName, 'A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)')
            [CompletionResult]::new('--wasm-opt-path', 'wasm-opt-path', [CompletionResultType]::ParameterName, 'A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)')
            [CompletionResult]::new('--rustup-path', 'rustup-path', [CompletionResultType]::ParameterName, 'The path to `rustup`')
            [CompletionResult]::new('--wasm-release-rustflags', 'wasm-release-rustflags', [CompletionResultType]::ParameterName, 'The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)')
            [CompletionResult]::new('--cargo-engine-args', 'cargo-engine-args', [CompletionResultType]::ParameterName, 'Any arguments to `cargo` when building for the engine-side')
            [CompletionResult]::new('--cargo-browser-args', 'cargo-browser-args', [CompletionResultType]::ParameterName, 'Any arguments to `cargo` when building for the browser-side')
            [CompletionResult]::new('--wasm-bindgen-args', 'wasm-bindgen-args', [CompletionResultType]::ParameterName, 'Any arguments to `wasm-bindgen`')
            [CompletionResult]::new('--wasm-opt-args', 'wasm-opt-args', [CompletionResultType]::ParameterName, 'Any arguments to `wasm-opt` (only run in release builds)')
            [CompletionResult]::new('--git-path', 'git-path', [CompletionResultType]::ParameterName, 'The path to `git` (for downloading custom templates for `perseus new`)')
            [CompletionResult]::new('--reload-server-host', 'reload-server-host', [CompletionResultType]::ParameterName, 'The host for the reload server (you should almost never change this)')
            [CompletionResult]::new('--reload-server-port', 'reload-server-port', [CompletionResultType]::ParameterName, 'The port for the reload server (you should almost never change this)')
            [CompletionResult]::new('--wasm-bindgen-version', 'wasm-bindgen-version', [CompletionResultType]::ParameterName, 'A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)')
            [CompletionResult]::new('--wasm-opt-version', 'wasm-opt-version', [CompletionResultType]::ParameterName, 'A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)')
            [CompletionResult]::new('-e', 'e', [CompletionResultType]::ParameterName, 'Export your app to purely static files (see `export`)')
            [CompletionResult]::new('--export-static', 'export-static', [CompletionResultType]::ParameterName, 'Export your app to purely static files (see `export`)')
            [CompletionResult]::new('--no-minify-js', 'no-minify-js', [CompletionResultType]::ParameterName, 'Don''t minify JavaScript (this will decrease performance)')
            [CompletionResult]::new('--sequential', 'sequential', [CompletionResultType]::ParameterName, 'If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)')
            [CompletionResult]::new('--no-browser-reload', 'no-browser-reload', [CompletionResultType]::ParameterName, 'Disable automatic browser reloading')
            [CompletionResult]::new('--no-system-tools-cache', 'no-system-tools-cache', [CompletionResultType]::ParameterName, 'Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging')
            [CompletionResult]::new('--disable-bundle-compression', 'disable-bundle-compression', [CompletionResultType]::ParameterName, 'Disable Brotli compression of JS and Wasm bundles (may degrade performance)')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'perseus;tinker' {
            [CompletionResult]::new('--cargo-engine-path', 'cargo-engine-path', [CompletionResultType]::ParameterName, 'The path to `cargo` when used for engine builds')
            [CompletionResult]::new('--cargo-browser-path', 'cargo-browser-path', [CompletionResultType]::ParameterName, 'The path to `cargo` when used for browser builds')
            [CompletionResult]::new('--wasm-bindgen-path', 'wasm-bindgen-path', [CompletionResultType]::ParameterName, 'A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)')
            [CompletionResult]::new('--wasm-opt-path', 'wasm-opt-path', [CompletionResultType]::ParameterName, 'A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)')
            [CompletionResult]::new('--rustup-path', 'rustup-path', [CompletionResultType]::ParameterName, 'The path to `rustup`')
            [CompletionResult]::new('--wasm-release-rustflags', 'wasm-release-rustflags', [CompletionResultType]::ParameterName, 'The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)')
            [CompletionResult]::new('--cargo-engine-args', 'cargo-engine-args', [CompletionResultType]::ParameterName, 'Any arguments to `cargo` when building for the engine-side')
            [CompletionResult]::new('--cargo-browser-args', 'cargo-browser-args', [CompletionResultType]::ParameterName, 'Any arguments to `cargo` when building for the browser-side')
            [CompletionResult]::new('--wasm-bindgen-args', 'wasm-bindgen-args', [CompletionResultType]::ParameterName, 'Any arguments to `wasm-bindgen`')
            [CompletionResult]::new('--wasm-opt-args', 'wasm-opt-args', [CompletionResultType]::ParameterName, 'Any arguments to `wasm-opt` (only run in release builds)')
            [CompletionResult]::new('--git-path', 'git-path', [CompletionResultType]::ParameterName, 'The path to `git` (for downloading custom templates for `perseus new`)')
            [CompletionResult]::new('--reload-server-host', 'reload-server-host', [CompletionResultType]::ParameterName, 'The host for the reload server (you should almost never change this)')
            [CompletionResult]::new('--reload-server-port', 'reload-server-port', [CompletionResultType]::ParameterName, 'The port for the reload server (you should almost never change this)')
            [CompletionResult]::new('--wasm-bindgen-version', 'wasm-bindgen-version', [CompletionResultType]::ParameterName, 'A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)')
            [CompletionResult]::new('--wasm-opt-version', 'wasm-opt-version', [CompletionResultType]::ParameterName, 'A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)')
            [CompletionResult]::new('--no-clean', 'no-clean', [CompletionResultType]::ParameterName, 'Don''t remove and recreate the `dist/` directory')
            [CompletionResult]::new('--sequential', 'sequential', [CompletionResultType]::ParameterName, 'If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)')
            [CompletionResult]::new('--no-browser-reload', 'no-browser-reload', [CompletionResultType]::ParameterName, 'Disable automatic browser reloading')
            [CompletionResult]::new('--no-system-tools-cache', 'no-system-tools-cache', [CompletionResultType]::ParameterName, 'Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging')
            [CompletionResult]::new('--disable-bundle-compression', 'disable-bundle-compression', [CompletionResultType]::ParameterName, 'Disable Brotli compression of JS and Wasm bundles (may degrade performance)')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'perseus;snoop' {
            [CompletionResult]::new('--cargo-engine-path', 'cargo-engine-path', [CompletionResultType]::ParameterName, 'The path to `cargo` when used for engine builds')
            [CompletionResult]::new('--cargo-browser-path', 'cargo-browser-path', [CompletionResultType]::ParameterName, 'The path to `cargo` when used for browser builds')
            [CompletionResult]::new('--wasm-bindgen-path', 'wasm-bindgen-path', [CompletionResultType]::ParameterName, 'A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)')
            [CompletionResult]::new('--wasm-opt-path', 'wasm-opt-path', [CompletionResultType]::ParameterName, 'A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)')
            [CompletionResult]::new('--rustup-path', 'rustup-path', [CompletionResultType]::ParameterName, 'The path to `rustup`')
            [CompletionResult]::new('--wasm-release-rustflags', 'wasm-release-rustflags', [CompletionResultType]::ParameterName, 'The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)')
            [CompletionResult]::new('--cargo-engine-args', 'cargo-engine-args', [CompletionResultType]::ParameterName, 'Any arguments to `cargo` when building for the engine-side')
            [CompletionResult]::new('--cargo-browser-args', 'cargo-browser-args', [CompletionResultType]::ParameterName, 'Any arguments to `cargo` when building for the browser-side')
            [CompletionResult]::new('--wasm-bindgen-args', 'wasm-bindgen-args', [CompletionResultType]::ParameterName, 'Any arguments to `wasm-bindgen`')
            [CompletionResult]::new('--wasm-opt-args', 'wasm-opt-args', [CompletionResultType]::ParameterName, 'Any arguments to `wasm-opt` (only run in release builds)')
            [CompletionResult]::new('--git-path', 'git-path', [CompletionResultType]::ParameterName, 'The path to `git` (for downloading custom templates for `perseus new`)')
            [CompletionResult]::new('--reload-server-host', 'reload-server-host', [CompletionResultType]::ParameterName, 'The host for the reload server (you should almost never change this)')
            [CompletionResult]::new('--reload-server-port', 'reload-server-port', [CompletionResultType]::ParameterName, 'The port for the reload server (you should almost never change this)')
            [CompletionResult]::new('--wasm-bindgen-version', 'wasm-bindgen-version', [CompletionResultType]::ParameterName, 'A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)')
            [CompletionResult]::new('--wasm-opt-version', 'wasm-opt-version', [CompletionResultType]::ParameterName, 'A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)')
            [CompletionResult]::new('--sequential', 'sequential', [CompletionResultType]::ParameterName, 'If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)')
            [CompletionResult]::new('--no-browser-reload', 'no-browser-reload', [CompletionResultType]::ParameterName, 'Disable automatic browser reloading')
            [CompletionResult]::new('--no-system-tools-cache', 'no-system-tools-cache', [CompletionResultType]::ParameterName, 'Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging')
            [CompletionResult]::new('--disable-bundle-compression', 'disable-bundle-compression', [CompletionResultType]::ParameterName, 'Disable Brotli compression of JS and Wasm bundles (may degrade performance)')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('build', 'build', [CompletionResultType]::ParameterValue, 'Snoops on the static generation process (this will let you see `dbg!` calls and the like)')
            [CompletionResult]::new('wasm-build', 'wasm-build', [CompletionResultType]::ParameterValue, 'Snoops on the Wasm building process (mostly for debugging errors)')
            [CompletionResult]::new('serve', 'serve', [CompletionResultType]::ParameterValue, 'Snoops on the server process (run `perseus build` before this)')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'perseus;snoop;build' {
            [CompletionResult]::new('--custom-watch', 'custom-watch', [CompletionResultType]::ParameterName, 'Marks a specific file/directory to be watched (directories will be recursively watched)')
            [CompletionResult]::new('--cargo-engine-path', 'cargo-engine-path', [CompletionResultType]::ParameterName, 'The path to `cargo` when used for engine builds')
            [CompletionResult]::new('--cargo-browser-path', 'cargo-browser-path', [CompletionResultType]::ParameterName, 'The path to `cargo` when used for browser builds')
            [CompletionResult]::new('--wasm-bindgen-path', 'wasm-bindgen-path', [CompletionResultType]::ParameterName, 'A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)')
            [CompletionResult]::new('--wasm-opt-path', 'wasm-opt-path', [CompletionResultType]::ParameterName, 'A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)')
            [CompletionResult]::new('--rustup-path', 'rustup-path', [CompletionResultType]::ParameterName, 'The path to `rustup`')
            [CompletionResult]::new('--wasm-release-rustflags', 'wasm-release-rustflags', [CompletionResultType]::ParameterName, 'The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)')
            [CompletionResult]::new('--cargo-engine-args', 'cargo-engine-args', [CompletionResultType]::ParameterName, 'Any arguments to `cargo` when building for the engine-side')
            [CompletionResult]::new('--cargo-browser-args', 'cargo-browser-args', [CompletionResultType]::ParameterName, 'Any arguments to `cargo` when building for the browser-side')
            [CompletionResult]::new('--wasm-bindgen-args', 'wasm-bindgen-args', [CompletionResultType]::ParameterName, 'Any arguments to `wasm-bindgen`')
            [CompletionResult]::new('--wasm-opt-args', 'wasm-opt-args', [CompletionResultType]::ParameterName, 'Any arguments to `wasm-opt` (only run in release builds)')
            [CompletionResult]::new('--git-path', 'git-path', [CompletionResultType]::ParameterName, 'The path to `git` (for downloading custom templates for `perseus new`)')
            [CompletionResult]::new('--reload-server-host', 'reload-server-host', [CompletionResultType]::ParameterName, 'The host for the reload server (you should almost never change this)')
            [CompletionResult]::new('--reload-server-port', 'reload-server-port', [CompletionResultType]::ParameterName, 'The port for the reload server (you should almost never change this)')
            [CompletionResult]::new('--wasm-bindgen-version', 'wasm-bindgen-version', [CompletionResultType]::ParameterName, 'A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)')
            [CompletionResult]::new('--wasm-opt-version', 'wasm-opt-version', [CompletionResultType]::ParameterName, 'A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)')
            [CompletionResult]::new('-w', 'w', [CompletionResultType]::ParameterName, 'Watch the files in your working directory for changes (excluding `target/` and `dist/`)')
            [CompletionResult]::new('--watch', 'watch', [CompletionResultType]::ParameterName, 'Watch the files in your working directory for changes (excluding `target/` and `dist/`)')
            [CompletionResult]::new('--sequential', 'sequential', [CompletionResultType]::ParameterName, 'If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)')
            [CompletionResult]::new('--no-browser-reload', 'no-browser-reload', [CompletionResultType]::ParameterName, 'Disable automatic browser reloading')
            [CompletionResult]::new('--no-system-tools-cache', 'no-system-tools-cache', [CompletionResultType]::ParameterName, 'Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging')
            [CompletionResult]::new('--disable-bundle-compression', 'disable-bundle-compression', [CompletionResultType]::ParameterName, 'Disable Brotli compression of JS and Wasm bundles (may degrade performance)')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'perseus;snoop;wasm-build' {
            [CompletionResult]::new('--custom-watch', 'custom-watch', [CompletionResultType]::ParameterName, 'Marks a specific file/directory to be watched (directories will be recursively watched)')
            [CompletionResult]::new('--cargo-engine-path', 'cargo-engine-path', [CompletionResultType]::ParameterName, 'The path to `cargo` when used for engine builds')
            [CompletionResult]::new('--cargo-browser-path', 'cargo-browser-path', [CompletionResultType]::ParameterName, 'The path to `cargo` when used for browser builds')
            [CompletionResult]::new('--wasm-bindgen-path', 'wasm-bindgen-path', [CompletionResultType]::ParameterName, 'A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)')
            [CompletionResult]::new('--wasm-opt-path', 'wasm-opt-path', [CompletionResultType]::ParameterName, 'A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)')
            [CompletionResult]::new('--rustup-path', 'rustup-path', [CompletionResultType]::ParameterName, 'The path to `rustup`')
            [CompletionResult]::new('--wasm-release-rustflags', 'wasm-release-rustflags', [CompletionResultType]::ParameterName, 'The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)')
            [CompletionResult]::new('--cargo-engine-args', 'cargo-engine-args', [CompletionResultType]::ParameterName, 'Any arguments to `cargo` when building for the engine-side')
            [CompletionResult]::new('--cargo-browser-args', 'cargo-browser-args', [CompletionResultType]::ParameterName, 'Any arguments to `cargo` when building for the browser-side')
            [CompletionResult]::new('--wasm-bindgen-args', 'wasm-bindgen-args', [CompletionResultType]::ParameterName, 'Any arguments to `wasm-bindgen`')
            [CompletionResult]::new('--wasm-opt-args', 'wasm-opt-args', [CompletionResultType]::ParameterName, 'Any arguments to `wasm-opt` (only run in release builds)')
            [CompletionResult]::new('--git-path', 'git-path', [CompletionResultType]::ParameterName, 'The path to `git` (for downloading custom templates for `perseus new`)')
            [CompletionResult]::new('--reload-server-host', 'reload-server-host', [CompletionResultType]::ParameterName, 'The host for the reload server (you should almost never change this)')
            [CompletionResult]::new('--reload-server-port', 'reload-server-port', [CompletionResultType]::ParameterName, 'The port for the reload server (you should almost never change this)')
            [CompletionResult]::new('--wasm-bindgen-version', 'wasm-bindgen-version', [CompletionResultType]::ParameterName, 'A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)')
            [CompletionResult]::new('--wasm-opt-version', 'wasm-opt-version', [CompletionResultType]::ParameterName, 'A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)')
            [CompletionResult]::new('-w', 'w', [CompletionResultType]::ParameterName, 'Watch the files in your working directory for changes (excluding `target/` and `dist/`)')
            [CompletionResult]::new('--watch', 'watch', [CompletionResultType]::ParameterName, 'Watch the files in your working directory for changes (excluding `target/` and `dist/`)')
            [CompletionResult]::new('--sequential', 'sequential', [CompletionResultType]::ParameterName, 'If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)')
            [CompletionResult]::new('--no-browser-reload', 'no-browser-reload', [CompletionResultType]::ParameterName, 'Disable automatic browser reloading')
            [CompletionResult]::new('--no-system-tools-cache', 'no-system-tools-cache', [CompletionResultType]::ParameterName, 'Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging')
            [CompletionResult]::new('--disable-bundle-compression', 'disable-bundle-compression', [CompletionResultType]::ParameterName, 'Disable Brotli compression of JS and Wasm bundles (may degrade performance)')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'perseus;snoop;serve' {
            [CompletionResult]::new('--host', 'host', [CompletionResultType]::ParameterName, 'Where to host your exported app')
            [CompletionResult]::new('--port', 'port', [CompletionResultType]::ParameterName, 'The port to host your exported app on')
            [CompletionResult]::new('--custom-watch', 'custom-watch', [CompletionResultType]::ParameterName, 'Marks a specific file/directory to be watched (directories will be recursively watched)')
            [CompletionResult]::new('--cargo-engine-path', 'cargo-engine-path', [CompletionResultType]::ParameterName, 'The path to `cargo` when used for engine builds')
            [CompletionResult]::new('--cargo-browser-path', 'cargo-browser-path', [CompletionResultType]::ParameterName, 'The path to `cargo` when used for browser builds')
            [CompletionResult]::new('--wasm-bindgen-path', 'wasm-bindgen-path', [CompletionResultType]::ParameterName, 'A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)')
            [CompletionResult]::new('--wasm-opt-path', 'wasm-opt-path', [CompletionResultType]::ParameterName, 'A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)')
            [CompletionResult]::new('--rustup-path', 'rustup-path', [CompletionResultType]::ParameterName, 'The path to `rustup`')
            [CompletionResult]::new('--wasm-release-rustflags', 'wasm-release-rustflags', [CompletionResultType]::ParameterName, 'The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)')
            [CompletionResult]::new('--cargo-engine-args', 'cargo-engine-args', [CompletionResultType]::ParameterName, 'Any arguments to `cargo` when building for the engine-side')
            [CompletionResult]::new('--cargo-browser-args', 'cargo-browser-args', [CompletionResultType]::ParameterName, 'Any arguments to `cargo` when building for the browser-side')
            [CompletionResult]::new('--wasm-bindgen-args', 'wasm-bindgen-args', [CompletionResultType]::ParameterName, 'Any arguments to `wasm-bindgen`')
            [CompletionResult]::new('--wasm-opt-args', 'wasm-opt-args', [CompletionResultType]::ParameterName, 'Any arguments to `wasm-opt` (only run in release builds)')
            [CompletionResult]::new('--git-path', 'git-path', [CompletionResultType]::ParameterName, 'The path to `git` (for downloading custom templates for `perseus new`)')
            [CompletionResult]::new('--reload-server-host', 'reload-server-host', [CompletionResultType]::ParameterName, 'The host for the reload server (you should almost never change this)')
            [CompletionResult]::new('--reload-server-port', 'reload-server-port', [CompletionResultType]::ParameterName, 'The port for the reload server (you should almost never change this)')
            [CompletionResult]::new('--wasm-bindgen-version', 'wasm-bindgen-version', [CompletionResultType]::ParameterName, 'A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)')
            [CompletionResult]::new('--wasm-opt-version', 'wasm-opt-version', [CompletionResultType]::ParameterName, 'A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)')
            [CompletionResult]::new('-w', 'w', [CompletionResultType]::ParameterName, 'Watch the files in your working directory for changes (excluding `target/` and `dist/`)')
            [CompletionResult]::new('--watch', 'watch', [CompletionResultType]::ParameterName, 'Watch the files in your working directory for changes (excluding `target/` and `dist/`)')
            [CompletionResult]::new('--sequential', 'sequential', [CompletionResultType]::ParameterName, 'If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)')
            [CompletionResult]::new('--no-browser-reload', 'no-browser-reload', [CompletionResultType]::ParameterName, 'Disable automatic browser reloading')
            [CompletionResult]::new('--no-system-tools-cache', 'no-system-tools-cache', [CompletionResultType]::ParameterName, 'Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging')
            [CompletionResult]::new('--disable-bundle-compression', 'disable-bundle-compression', [CompletionResultType]::ParameterName, 'Disable Brotli compression of JS and Wasm bundles (may degrade performance)')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'perseus;snoop;help' {
            [CompletionResult]::new('build', 'build', [CompletionResultType]::ParameterValue, 'Snoops on the static generation process (this will let you see `dbg!` calls and the like)')
            [CompletionResult]::new('wasm-build', 'wasm-build', [CompletionResultType]::ParameterValue, 'Snoops on the Wasm building process (mostly for debugging errors)')
            [CompletionResult]::new('serve', 'serve', [CompletionResultType]::ParameterValue, 'Snoops on the server process (run `perseus build` before this)')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'perseus;snoop;help;build' {
            break
        }
        'perseus;snoop;help;wasm-build' {
            break
        }
        'perseus;snoop;help;serve' {
            break
        }
        'perseus;snoop;help;help' {
            break
        }
        'perseus;new' {
            [CompletionResult]::new('-t', 't', [CompletionResultType]::ParameterName, 'An optional custom URL to a Git repository to be used as a custom template (note that custom templates will not respect your project''s name). This can be followed with `@branch` to fetch from `branch` rather than the default')
            [CompletionResult]::new('--template', 'template', [CompletionResultType]::ParameterName, 'An optional custom URL to a Git repository to be used as a custom template (note that custom templates will not respect your project''s name). This can be followed with `@branch` to fetch from `branch` rather than the default')
            [CompletionResult]::new('--dir', 'dir', [CompletionResultType]::ParameterName, 'The path to a custom directory to create (if this is not provided, the project name will be used by default)')
            [CompletionResult]::new('--cargo-engine-path', 'cargo-engine-path', [CompletionResultType]::ParameterName, 'The path to `cargo` when used for engine builds')
            [CompletionResult]::new('--cargo-browser-path', 'cargo-browser-path', [CompletionResultType]::ParameterName, 'The path to `cargo` when used for browser builds')
            [CompletionResult]::new('--wasm-bindgen-path', 'wasm-bindgen-path', [CompletionResultType]::ParameterName, 'A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)')
            [CompletionResult]::new('--wasm-opt-path', 'wasm-opt-path', [CompletionResultType]::ParameterName, 'A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)')
            [CompletionResult]::new('--rustup-path', 'rustup-path', [CompletionResultType]::ParameterName, 'The path to `rustup`')
            [CompletionResult]::new('--wasm-release-rustflags', 'wasm-release-rustflags', [CompletionResultType]::ParameterName, 'The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)')
            [CompletionResult]::new('--cargo-engine-args', 'cargo-engine-args', [CompletionResultType]::ParameterName, 'Any arguments to `cargo` when building for the engine-side')
            [CompletionResult]::new('--cargo-browser-args', 'cargo-browser-args', [CompletionResultType]::ParameterName, 'Any arguments to `cargo` when building for the browser-side')
            [CompletionResult]::new('--wasm-bindgen-args', 'wasm-bindgen-args', [CompletionResultType]::ParameterName, 'Any arguments to `wasm-bindgen`')
            [CompletionResult]::new('--wasm-opt-args', 'wasm-opt-args', [CompletionResultType]::ParameterName, 'Any arguments to `wasm-opt` (only run in release builds)')
            [CompletionResult]::new('--git-path', 'git-path', [CompletionResultType]::ParameterName, 'The path to `git` (for downloading custom templates for `perseus new`)')
            [CompletionResult]::new('--reload-server-host', 'reload-server-host', [CompletionResultType]::ParameterName, 'The host for the reload server (you should almost never change this)')
            [CompletionResult]::new('--reload-server-port', 'reload-server-port', [CompletionResultType]::ParameterName, 'The port for the reload server (you should almost never change this)')
            [CompletionResult]::new('--wasm-bindgen-version', 'wasm-bindgen-version', [CompletionResultType]::ParameterName, 'A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)')
            [CompletionResult]::new('--wasm-opt-version', 'wasm-opt-version', [CompletionResultType]::ParameterName, 'A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)')
            [CompletionResult]::new('--sequential', 'sequential', [CompletionResultType]::ParameterName, 'If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)')
            [CompletionResult]::new('--no-browser-reload', 'no-browser-reload', [CompletionResultType]::ParameterName, 'Disable automatic browser reloading')
            [CompletionResult]::new('--no-system-tools-cache', 'no-system-tools-cache', [CompletionResultType]::ParameterName, 'Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging')
            [CompletionResult]::new('--disable-bundle-compression', 'disable-bundle-compression', [CompletionResultType]::ParameterName, 'Disable Brotli compression of JS and Wasm bundles (may degrade performance)')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'perseus;init' {
            [CompletionResult]::new('--cargo-engine-path', 'cargo-engine-path', [CompletionResultType]::ParameterName, 'The path to `cargo` when used for engine builds')
            [CompletionResult]::new('--cargo-browser-path', 'cargo-browser-path', [CompletionResultType]::ParameterName, 'The path to `cargo` when used for browser builds')
            [CompletionResult]::new('--wasm-bindgen-path', 'wasm-bindgen-path', [CompletionResultType]::ParameterName, 'A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)')
            [CompletionResult]::new('--wasm-opt-path', 'wasm-opt-path', [CompletionResultType]::ParameterName, 'A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)')
            [CompletionResult]::new('--rustup-path', 'rustup-path', [CompletionResultType]::ParameterName, 'The path to `rustup`')
            [CompletionResult]::new('--wasm-release-rustflags', 'wasm-release-rustflags', [CompletionResultType]::ParameterName, 'The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)')
            [CompletionResult]::new('--cargo-engine-args', 'cargo-engine-args', [CompletionResultType]::ParameterName, 'Any arguments to `cargo` when building for the engine-side')
            [CompletionResult]::new('--cargo-browser-args', 'cargo-browser-args', [CompletionResultType]::ParameterName, 'Any arguments to `cargo` when building for the browser-side')
            [CompletionResult]::new('--wasm-bindgen-args', 'wasm-bindgen-args', [CompletionResultType]::ParameterName, 'Any arguments to `wasm-bindgen`')
            [CompletionResult]::new('--wasm-opt-args', 'wasm-opt-args', [CompletionResultType]::ParameterName, 'Any arguments to `wasm-opt` (only run in release builds)')
            [CompletionResult]::new('--git-path', 'git-path', [CompletionResultType]::ParameterName, 'The path to `git` (for downloading custom templates for `perseus new`)')
            [CompletionResult]::new('--reload-server-host', 'reload-server-host', [CompletionResultType]::ParameterName, 'The host for the reload server (you should almost never change this)')
            [CompletionResult]::new('--reload-server-port', 'reload-server-port', [CompletionResultType]::ParameterName, 'The port for the reload server (you should almost never change this)')
            [CompletionResult]::new('--wasm-bindgen-version', 'wasm-bindgen-version', [CompletionResultType]::ParameterName, 'A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)')
            [CompletionResult]::new('--wasm-opt-version', 'wasm-opt-version', [CompletionResultType]::ParameterName, 'A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)')
            [CompletionResult]::new('--sequential', 'sequential', [CompletionResultType]::ParameterName, 'If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)')
            [CompletionResult]::new('--no-browser-reload', 'no-browser-reload', [CompletionResultType]::ParameterName, 'Disable automatic browser reloading')
            [CompletionResult]::new('--no-system-tools-cache', 'no-system-tools-cache', [CompletionResultType]::ParameterName, 'Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging')
            [CompletionResult]::new('--disable-bundle-compression', 'disable-bundle-compression', [CompletionResultType]::ParameterName, 'Disable Brotli compression of JS and Wasm bundles (may degrade performance)')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'perseus;check' {
            [CompletionResult]::new('--custom-watch', 'custom-watch', [CompletionResultType]::ParameterName, 'Marks a specific file/directory to be watched (directories will be recursively watched)')
            [CompletionResult]::new('--cargo-engine-path', 'cargo-engine-path', [CompletionResultType]::ParameterName, 'The path to `cargo` when used for engine builds')
            [CompletionResult]::new('--cargo-browser-path', 'cargo-browser-path', [CompletionResultType]::ParameterName, 'The path to `cargo` when used for browser builds')
            [CompletionResult]::new('--wasm-bindgen-path', 'wasm-bindgen-path', [CompletionResultType]::ParameterName, 'A path to `wasm-bindgen`, if you want to use a local installation (note that the CLI will install it locally for you by default)')
            [CompletionResult]::new('--wasm-opt-path', 'wasm-opt-path', [CompletionResultType]::ParameterName, 'A path to `wasm-opt`, if you want to use a local installation (note that the CLI will install it locally for you by default)')
            [CompletionResult]::new('--rustup-path', 'rustup-path', [CompletionResultType]::ParameterName, 'The path to `rustup`')
            [CompletionResult]::new('--wasm-release-rustflags', 'wasm-release-rustflags', [CompletionResultType]::ParameterName, 'The value of `RUSTFLAGS` when building for Wasm in release mode (this will not impact internal target-gating)')
            [CompletionResult]::new('--cargo-engine-args', 'cargo-engine-args', [CompletionResultType]::ParameterName, 'Any arguments to `cargo` when building for the engine-side')
            [CompletionResult]::new('--cargo-browser-args', 'cargo-browser-args', [CompletionResultType]::ParameterName, 'Any arguments to `cargo` when building for the browser-side')
            [CompletionResult]::new('--wasm-bindgen-args', 'wasm-bindgen-args', [CompletionResultType]::ParameterName, 'Any arguments to `wasm-bindgen`')
            [CompletionResult]::new('--wasm-opt-args', 'wasm-opt-args', [CompletionResultType]::ParameterName, 'Any arguments to `wasm-opt` (only run in release builds)')
            [CompletionResult]::new('--git-path', 'git-path', [CompletionResultType]::ParameterName, 'The path to `git` (for downloading custom templates for `perseus new`)')
            [CompletionResult]::new('--reload-server-host', 'reload-server-host', [CompletionResultType]::ParameterName, 'The host for the reload server (you should almost never change this)')
            [CompletionResult]::new('--reload-server-port', 'reload-server-port', [CompletionResultType]::ParameterName, 'The port for the reload server (you should almost never change this)')
            [CompletionResult]::new('--wasm-bindgen-version', 'wasm-bindgen-version', [CompletionResultType]::ParameterName, 'A custom version of `wasm-bindgen` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)')
            [CompletionResult]::new('--wasm-opt-version', 'wasm-opt-version', [CompletionResultType]::ParameterName, 'A custom version of `wasm-opt` to use (defaults to the latest installed version, and after that the latest available from GitHub; update to latest can be forced with `latest`)')
            [CompletionResult]::new('-w', 'w', [CompletionResultType]::ParameterName, 'Watch the files in your working directory for changes (excluding `target/` and `dist/`)')
            [CompletionResult]::new('--watch', 'watch', [CompletionResultType]::ParameterName, 'Watch the files in your working directory for changes (excluding `target/` and `dist/`)')
            [CompletionResult]::new('-g', 'g', [CompletionResultType]::ParameterName, 'Make sure the app''s page generation works properly (this will take much longer, but almost guarantees that your app will actually build); use this to catch errors in build state and the like')
            [CompletionResult]::new('--generate', 'generate', [CompletionResultType]::ParameterName, 'Make sure the app''s page generation works properly (this will take much longer, but almost guarantees that your app will actually build); use this to catch errors in build state and the like')
            [CompletionResult]::new('--sequential', 'sequential', [CompletionResultType]::ParameterName, 'If this is set, commands will be run sequentially rather than in parallel (slows down operations, but reduces memory usage)')
            [CompletionResult]::new('--no-browser-reload', 'no-browser-reload', [CompletionResultType]::ParameterName, 'Disable automatic browser reloading')
            [CompletionResult]::new('--no-system-tools-cache', 'no-system-tools-cache', [CompletionResultType]::ParameterName, 'Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you should set this for CI)')
            [CompletionResult]::new('--verbose', 'verbose', [CompletionResultType]::ParameterName, 'Shows the logs from building and serving your app no matter what (the default is to only show them on a compilation/build failure); this is intended mainly for end-to-end debugging, although the `snoop` commands are more useful for targeted debugging')
            [CompletionResult]::new('--disable-bundle-compression', 'disable-bundle-compression', [CompletionResultType]::ParameterName, 'Disable Brotli compression of JS and Wasm bundles (may degrade performance)')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'perseus;help' {
            [CompletionResult]::new('build', 'build', [CompletionResultType]::ParameterValue, 'Builds your app')
            [CompletionResult]::new('export-error-page', 'export-error-page', [CompletionResultType]::ParameterValue, 'Exports an error page for the given HTTP status code')
            [CompletionResult]::new('export', 'export', [CompletionResultType]::ParameterValue, 'Exports your app to purely static files')
            [CompletionResult]::new('serve', 'serve', [CompletionResultType]::ParameterValue, 'Serves your app')
            [CompletionResult]::new('test', 'test', [CompletionResultType]::ParameterValue, 'Serves your app as `perseus serve` does, but puts it in testing mode')
            [CompletionResult]::new('clean', 'clean', [CompletionResultType]::ParameterValue, 'Removes build artifacts in the `dist/` directory')
            [CompletionResult]::new('deploy', 'deploy', [CompletionResultType]::ParameterValue, 'Packages your app for deployment')
            [CompletionResult]::new('tinker', 'tinker', [CompletionResultType]::ParameterValue, 'Runs the `tinker` action of plugins, which lets them modify the Perseus engine')
            [CompletionResult]::new('snoop', 'snoop', [CompletionResultType]::ParameterValue, 'Runs one of the underlying commands that builds your app, allowing you to see more detailed logs')
            [CompletionResult]::new('new', 'new', [CompletionResultType]::ParameterValue, 'Creates a new Perseus project in a directory of the given name, which will be created in the current path')
            [CompletionResult]::new('init', 'init', [CompletionResultType]::ParameterValue, 'Initializes a new Perseus project in the current directory')
            [CompletionResult]::new('check', 'check', [CompletionResultType]::ParameterValue, 'Checks if your app builds properly for both the engine-side and the browser-side')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'perseus;help;build' {
            break
        }
        'perseus;help;export-error-page' {
            break
        }
        'perseus;help;export' {
            break
        }
        'perseus;help;serve' {
            break
        }
        'perseus;help;test' {
            break
        }
        'perseus;help;clean' {
            break
        }
        'perseus;help;deploy' {
            break
        }
        'perseus;help;tinker' {
            break
        }
        'perseus;help;snoop' {
            [CompletionResult]::new('build', 'build', [CompletionResultType]::ParameterValue, 'Snoops on the static generation process (this will let you see `dbg!` calls and the like)')
            [CompletionResult]::new('wasm-build', 'wasm-build', [CompletionResultType]::ParameterValue, 'Snoops on the Wasm building process (mostly for debugging errors)')
            [CompletionResult]::new('serve', 'serve', [CompletionResultType]::ParameterValue, 'Snoops on the server process (run `perseus build` before this)')
            break
        }
        'perseus;help;snoop;build' {
            break
        }
        'perseus;help;snoop;wasm-build' {
            break
        }
        'perseus;help;snoop;serve' {
            break
        }
        'perseus;help;new' {
            break
        }
        'perseus;help;init' {
            break
        }
        'perseus;help;check' {
            break
        }
        'perseus;help;help' {
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
