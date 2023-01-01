#![allow(missing_docs)] // Prevents double-documenting some things

use crate::PERSEUS_VERSION;
use clap::Parser;

// The documentation for the `Opts` struct will appear in the help page, hence
// the lack of punctuation and the lowercasing in places

/// The command-line interface for Perseus, a super-fast WebAssembly frontend
/// development framework!
#[derive(Parser, Clone)]
#[clap(version = PERSEUS_VERSION)]
// #[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: Subcommand,
    // All the following arguments are global, and can provided to any subcommand
    /// The path to `cargo` when used for engine builds
    #[clap(long, default_value = "cargo", global = true)]
    pub cargo_engine_path: String,
    /// The path to `cargo` when used for browser builds
    #[clap(long, default_value = "cargo", global = true)]
    pub cargo_browser_path: String,
    /// A path to `wasm-bindgen`, if you want to use a local installation (note
    /// that the CLI will install it locally for you by default)
    #[clap(long, global = true)]
    pub wasm_bindgen_path: Option<String>,
    /// A path to `wasm-opt`, if you want to use a local installation (note that
    /// the CLI will install it locally for you by default)
    #[clap(long, global = true)]
    pub wasm_opt_path: Option<String>,
    /// The path to `rustup`
    #[clap(long, default_value = "rustup", global = true)]
    pub rustup_path: String,
    /// The value of `RUSTFLAGS` when building for Wasm in release mode (this
    /// will not impact internal target-gating)
    #[clap(
        long,
        default_value = "-C opt-level=z -C codegen-units=1",
        global = true
    )]
    pub wasm_release_rustflags: String,
    /// Any arguments to `cargo` when building for the engine-side
    #[clap(long, default_value = "", global = true)]
    pub cargo_engine_args: String,
    /// Any arguments to `cargo` when building for the browser-side
    #[clap(long, default_value = "", global = true)]
    pub cargo_browser_args: String,
    /// Any arguments to `wasm-bindgen`
    #[clap(long, default_value = "", global = true)]
    pub wasm_bindgen_args: String,
    /// Any arguments to `wasm-opt` (only run in release builds)
    #[clap(long, default_value = "-Oz", global = true)]
    pub wasm_opt_args: String,
    /// The path to `git` (for downloading custom templates for `perseus new`)
    #[clap(long, default_value = "git", global = true)]
    pub git_path: String,
    /// The host for the reload server (you should almost never change this)
    #[clap(long, default_value = "localhost", global = true)]
    pub reload_server_host: String,
    /// The port for the reload server (you should almost never change this)
    #[clap(long, default_value = "3100", global = true)]
    pub reload_server_port: u16,
    /// If this is set, commands will be run sequentially rather than in
    /// parallel (slows down operations, but reduces memory usage)
    #[clap(long, global = true)]
    pub sequential: bool,
    /// Disable automatic browser reloading
    #[clap(long, global = true)]
    pub no_browser_reload: bool,
    /// A custom version of `wasm-bindgen` to use (defaults to the latest
    /// installed version, and after that the latest available from GitHub;
    /// update to latest can be forced with `latest`)
    #[clap(long, global = true)]
    pub wasm_bindgen_version: Option<String>,
    /// A custom version of `wasm-opt` to use (defaults to the latest installed
    /// version, and after that the latest available from GitHub; update to
    /// latest can be forced with `latest`)
    #[clap(long, global = true)]
    pub wasm_opt_version: Option<String>,
    /// Disables the system-wide tools cache in `~/.cargo/perseus_tools/` (you
    /// should set this for CI)
    #[clap(long, global = true)]
    pub no_system_tools_cache: bool,
}

#[derive(Parser, Clone)]
pub enum Subcommand {
    Build(BuildOpts),
    ExportErrorPage(ExportErrorPageOpts),
    Export(ExportOpts),
    Serve(ServeOpts),
    /// Serves your app as `perseus serve` does, but puts it in testing mode
    Test(ServeOpts),
    /// Removes build artifacts in the `dist/` directory
    Clean,
    Deploy(DeployOpts),
    Tinker(TinkerOpts),
    /// Runs one of the underlying commands that builds your app, allowing you
    /// to see more detailed logs
    #[clap(subcommand)]
    Snoop(SnoopSubcommand),
    New(NewOpts),
    Init(InitOpts),
    /// Checks if your app builds properly for both the engine-side and the
    /// browser-side
    Check(CheckOpts),
}
/// Builds your app
#[derive(Parser, Clone)]
pub struct BuildOpts {
    /// Build for production
    #[clap(long)]
    pub release: bool,
}
/// Exports your app to purely static files
#[derive(Parser, Clone)]
pub struct ExportOpts {
    /// Export for production
    #[clap(long)]
    pub release: bool,
    /// Serve the generated static files locally
    #[clap(short, long)]
    pub serve: bool,
    /// Where to host your exported app
    #[clap(long, default_value = "127.0.0.1")]
    pub host: String,
    /// The port to host your exported app on
    #[clap(long, default_value = "8080")]
    pub port: u16,
    /// Watch the files in your working directory for changes (excluding
    /// `target/` and `dist/`)
    #[clap(short, long)]
    pub watch: bool,
    /// Marks a specific file/directory to be watched (directories will be
    /// recursively watched)
    #[clap(long)]
    pub custom_watch: Vec<String>,
}
/// Exports an error page for the given HTTP status code
#[derive(Parser, Clone)]
pub struct ExportErrorPageOpts {
    #[clap(short, long)]
    pub code: String,
    #[clap(short, long)]
    pub output: String,
}
/// Serves your app (set the `$HOST` and `$PORT` environment variables to change
/// the location it's served at)
#[derive(Parser, Clone)]
pub struct ServeOpts {
    /// Don't run the final binary, but print its location instead as the last
    /// line of output
    #[clap(long)]
    pub no_run: bool,
    /// Only build the server, and use the results of a previous `perseus build`
    #[clap(long)]
    pub no_build: bool,
    /// Build and serve for production
    #[clap(long)]
    pub release: bool,
    /// Make the final binary standalone (this is used in `perseus deploy` only,
    /// don't manually invoke it unless you have a good reason!)
    #[clap(long)]
    pub standalone: bool,
    /// Watch the files in your working directory for changes (excluding
    /// `target/` and `dist/`)
    #[clap(short, long)]
    pub watch: bool,
    /// Marks a specific file/directory to be watched (directories will be
    /// recursively watched)
    #[clap(long)]
    pub custom_watch: Vec<String>,
    /// Where to host your exported app
    #[clap(long, default_value = "127.0.0.1")]
    pub host: String,
    /// The port to host your exported app on
    #[clap(long, default_value = "8080")]
    pub port: u16,
}
/// Packages your app for deployment
#[derive(Parser, Clone)]
pub struct DeployOpts {
    /// Change the output from `pkg/` to somewhere else
    #[clap(short, long, default_value = "pkg")]
    pub output: String,
    /// Export you app to purely static files (see `export`)
    #[clap(short, long)]
    pub export_static: bool,
}
/// Runs the `tinker` action of plugins, which lets them modify the Perseus
/// engine
#[derive(Parser, Clone)]
pub struct TinkerOpts {
    /// Don't remove and recreate the `dist/` directory
    #[clap(long)]
    pub no_clean: bool,
}
/// Creates a new Perseus project in a directory of the given name, which will
/// be created in the current path
#[derive(Parser, Clone)]
pub struct NewOpts {
    /// The name of the new project, which will also be used for the directory
    #[clap(value_parser)]
    pub name: String,
    /// An optional custom URL to a Git repository to be used as a custom
    /// template (note that custom templates will not respect your project's
    /// name). This can be followed with `@branch` to fetch from `branch`
    /// rather than the default
    #[clap(short, long)]
    pub template: Option<String>,
    /// The path to a custom directory to create (if this is not provided, the
    /// project name will be used by default)
    #[clap(long)]
    pub dir: Option<String>,
}
/// Initializes a new Perseus project in the current directory
#[derive(Parser, Clone)]
pub struct InitOpts {
    /// The name of the new project
    #[clap(value_parser)]
    pub name: String,
}

#[derive(Parser, Clone)]
pub enum SnoopSubcommand {
    /// Snoops on the static generation process (this will let you see `dbg!`
    /// calls and the like)
    Build,
    /// Snoops on the Wasm building process (mostly for debugging errors)
    WasmBuild,
    /// Snoops on the server process (run `perseus build` before this)
    Serve(SnoopServeOpts),
}

#[derive(Parser, Clone)]
pub struct SnoopServeOpts {
    /// Where to host your exported app
    #[clap(long, default_value = "127.0.0.1")]
    pub host: String,
    /// The port to host your exported app on
    #[clap(long, default_value = "8080")]
    pub port: u16,
}

#[derive(Parser, Clone)]
pub struct CheckOpts {
    /// Watch the files in your working directory for changes (excluding
    /// `target/` and `dist/`)
    #[clap(short, long)]
    pub watch: bool,
    /// Marks a specific file/directory to be watched (directories will be
    /// recursively watched)
    #[clap(long)]
    pub custom_watch: Vec<String>,
    /// Make sure the app's page generation works properly (this will take much
    /// longer, but almost guarantees that your app will actually build);
    /// use this to catch errors in build state and the like
    #[clap(short, long)]
    pub generate: bool,
}
