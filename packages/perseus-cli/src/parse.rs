#![allow(missing_docs)] // Prevents double-documenting some things

use crate::PERSEUS_VERSION;
use clap::Parser;

// The documentation for the `Opts` struct will appear in the help page, hence
// the lack of puncutation and the lowercasing in places

/// The command-line interface for Perseus, a super-fast WebAssembly frontend
/// development framework!
#[derive(Parser)]
#[clap(version = PERSEUS_VERSION)]
// #[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: Subcommand,
}

#[derive(Parser)]
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
}
/// Builds your app
#[derive(Parser)]
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
    /// Watch the files in your working directory for changes (exluding
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
    /// Watch the files in your working directory for changes (exluding
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
#[derive(Parser)]
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
#[derive(Parser)]
pub struct TinkerOpts {
    /// Don't remove and recreate the `dist/` directory
    #[clap(long)]
    pub no_clean: bool,
}

#[derive(Parser)]
pub enum SnoopSubcommand {
    /// Snoops on the static generation process (this will let you see `dbg!`
    /// calls and the like)
    Build,
    /// Snoops on the Wasm building process (mostly for debugging errors)
    WasmBuild(SnoopWasmOpts),
    /// Snoops on the server process (run `perseus build` before this)
    Serve(SnoopServeOpts),
}

#[derive(Parser)]
pub struct SnoopWasmOpts {
    /// Produce a profiling build (for use with `twiggy` and the like)
    #[clap(short, long)]
    pub profiling: bool,
}

#[derive(Parser)]
pub struct SnoopServeOpts {
    /// Where to host your exported app
    #[clap(long, default_value = "127.0.0.1")]
    pub host: String,
    /// The port to host your exported app on
    #[clap(long, default_value = "8080")]
    pub port: u16,
}
